import time
from dataclasses import dataclass
import ultralytics
import tomllib
import cv2
import numpy as np

ultralytics.checks()
# model = ultralytics.YOLO("yolov8m-seg_saved_model/yolov8m-seg_full_integer_quant_edgetpu.tflite")
model = ultralytics.YOLO("yolov8m-seg.pt", verbose=False)


@dataclass
class CameraSet:
    position: str
    front_src: str
    hand_src: str


morph_kernel = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (7, 7))


class RTSPMixerConfig:
    def __init__(self, file):
        with open(file, 'rb') as f:
            data = tomllib.load(f)

        self.left = CameraSet(
            "l",
            data["camera"]["front"]["l"]["src"],
            data["camera"]["hand"]["l"]["src"]
        )

        self.right = CameraSet(
            "r",
            data["camera"]["front"]["r"]["src"],
            data["camera"]["hand"]["r"]["src"]
        )


class ImageCapture:
    def __init__(self, src):
        self.cap = cv2.imread(src)

    def read(self):
        return True, self.cap


def crop_square(img, size, interpolation=cv2.INTER_AREA):
    h, w = img.shape[:2]
    min_size = np.amin([h, w])

    # Centralize and crop
    crop_img = img[int(h / 2 - min_size / 2):int(h / 2 + min_size / 2),
               int(w / 2 - min_size / 2):int(w / 2 + min_size / 2)]
    resized = cv2.resize(crop_img, (size, size), interpolation=interpolation)

    return resized


def white_balance(img):
    result = cv2.cvtColor(img, cv2.COLOR_BGR2LAB)
    avg_a = np.average(result[:, :, 1])
    avg_b = np.average(result[:, :, 2])
    result[:, :, 1] = result[:, :, 1] - ((avg_a - 128) * (result[:, :, 0] / 255.0) * 1.1)
    result[:, :, 2] = result[:, :, 2] - ((avg_b - 128) * (result[:, :, 0] / 255.0) * 1.1)
    result = cv2.cvtColor(result, cv2.COLOR_LAB2BGR)
    return result


def hand_img(img):
    img = cv2.bilateralFilter(img, d=9, sigmaColor=75, sigmaSpace=75)
    return white_balance(img)


def person_mask(img, last_mask=None):
    result = model.predict(img, verbose=False, conf=0.25)[0]
    cumulative_mask = np.zeros((480, 640), np.uint8)
    if result.masks is None:
        if last_mask is not None:
            return last_mask
        return crop_square(cumulative_mask, 350)

    for ci, mask in enumerate(result.masks.cpu().data.numpy()):
        box = result.boxes.cls.tolist()[ci]
        name = result.names[box]
        if name != "person" and name != "clock":
            continue
        # mask = result.masks.cpu().data.numpy()[0]
        cumulative_mask = np.bitwise_or(cumulative_mask, (mask * 255).astype(np.uint8))

    # mask = result.masks.cpu().data.numpy()[0]
    return crop_square(cumulative_mask, 350)


def final_mask(img, last_mask=None):
    mask = person_mask(img, last_mask)
    return cv2.morphologyEx(mask, cv2.MORPH_ERODE, morph_kernel)


class Streamer:
    def __init__(self, cam_set):
        self.front_str = ImageCapture(cam_set.front_src) if cam_set.front_src.startswith("./") else cv2.VideoCapture(cam_set.front_src)
        self.hand_str = ImageCapture(cam_set.hand_src) if cam_set.hand_src.startswith("./") else cv2.VideoCapture(
            cam_set.hand_src)
        self.mask = None
        self.maskimg = None
        self.handimg = None
        self.load_config = 2
        self.nth = self.load_config
        self.prev = None

    def resolve(self):
        front_ret, front = self.front_str.read()
        hand_ret, hand = self.hand_str.read()

        if not front_ret or not hand_ret:
            return None

        front_scaled = crop_square(front, 350)

        if self.nth % self.load_config == 0:
            self.handimg = hand
            mimg = hand_img(hand)
            mask = final_mask(mimg, self.mask)
            mask = cv2.blur(mask, (5, 5))
            self.mask = mask
            self.maskimg = mimg
            self.nth = 1

            hand_scaled = crop_square(hand, 350, cv2.INTER_CUBIC)
            self.prev = hand_scaled
        else:
            self.nth += 1

        final_image = front_scaled.copy()
        cv2.copyTo(self.prev, self.mask, final_image)
        return final_image


if __name__ == '__main__':
    config = RTSPMixerConfig("config.toml")
    left = Streamer(config.left)
    right = Streamer(config.right)
    while True:
        left_frame = left.resolve()
        right_frame = right.resolve()

        if left_frame is None or right_frame is None:
            print(type(left_frame), type(right_frame))
            continue

        dst = 30
        between = np.zeros((350, dst, 3), np.uint8)

        stack = cv2.hconcat([left_frame, between, right_frame])
        mstack = cv2.hconcat([
            cv2.vconcat([
                crop_square(left.handimg, 150),
                crop_square(left.maskimg, 150),
                crop_square(cv2.cvtColor(left.mask, cv2.COLOR_GRAY2BGR), 150)
            ]),
            cv2.vconcat([
                crop_square(right.handimg, 150),
                crop_square(right.maskimg, 150),
                crop_square(cv2.cvtColor(right.mask, cv2.COLOR_GRAY2BGR), 150)
            ])
        ])

        cv2.imshow("frame", stack)
        cv2.imshow("maskimg", mstack)
        key = cv2.waitKey(1)

        if key & 0xFF == ord("q"):
            break
