import pickle
import time

import cv2
import numpy as np


def postprocess_right(img, between, shift_correction):
    # image has weird blue and red channels, swap them
    img = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)

    # max height is 700
    if img.shape[0] > 700:
        img = cv2.resize(img, (int(700 * img.shape[1] / img.shape[0]), 700))

    between = between + 10000 - shift_correction
    if between >= 0:
        # add a black bar left
        img = cv2.copyMakeBorder(img, 0, 0, between, 0, cv2.BORDER_CONSTANT, value=[0, 0, 0])
    else:
        # crop left (max: width)
        crop_w = min(-between, img.shape[1] - 1)
        img = img[:, crop_w:]
        # add a black bar right
        img = cv2.copyMakeBorder(img, 0, 0, 0, -between, cv2.BORDER_CONSTANT, value=[0, 0, 0])

    return img


def postprocess_left(img, between):
    # image has weird blue and red channels, swap them
    img = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)

    # max height is 700
    if img.shape[0] > 700:
        img = cv2.resize(img, (int(700 * img.shape[1] / img.shape[0]), 700))

    between = between + 10000
    if between >= 0:
        # add a black bar right
        img = cv2.copyMakeBorder(img, 0, 0, 0, between, cv2.BORDER_CONSTANT, value=[0, 0, 0])
    else:
        # crop right (max: width)
        crop_w = min(-between, img.shape[1] - 1)
        img = img[:, :-crop_w]
        # add a black bar left
        img = cv2.copyMakeBorder(img, 0, 0, -between, 0, cv2.BORDER_CONSTANT, value=[0, 0, 0])

    return img


class Message:
    def __init__(self):
        self.between = 0
        self.shift_correction = 0
        self.left = None
        self.right = None

        self.preserialized = None

    def encode_numpy_ndarray(self, ndarray):
        return ndarray.dumps()

    def decode_numpy_ndarray(self, ndarray_bytes):
        return pickle.loads(ndarray_bytes)

    def read(self, b: bytes):
        before = time.time_ns()
        # first int is between
        self.between = -int.from_bytes(b[:4], byteorder='big')
        self.shift_correction = int.from_bytes(b[4:8], byteorder='big')
        # next int is length of left
        left_len = int.from_bytes(b[8:12], byteorder='big')
        # next int is length of right
        right_len = int.from_bytes(b[12:16], byteorder='big')

        # left is next left_len bytes
        left_bytes = b[16:16 + left_len]
        # right is next right_len bytes
        right_bytes = b[16 + left_len:16 + left_len + right_len]

        self.left = self.decode_numpy_ndarray(left_bytes)
        self.right = self.decode_numpy_ndarray(right_bytes)
        after = time.time_ns()

        return after - before

    def postprocess(self):
        self.left = postprocess_left(self.left, self.between)
        self.right = postprocess_right(self.right, self.between, self.shift_correction)

    def preserialize(self):
        # serialize between
        # left_bytes = cv2.imencode('.png', self.left)[1].tobytes()
        # right_bytes = cv2.imencode('.png', self.right)[1].tobytes()
        left_bytes = self.encode_numpy_ndarray(self.left)
        right_bytes = self.encode_numpy_ndarray(self.right)

        b = (-self.between).to_bytes(4, byteorder='big')
        b += self.shift_correction.to_bytes(4, byteorder='big')

        # serialize left length
        b += len(left_bytes).to_bytes(4, byteorder='big')
        # serialize right length
        b += len(right_bytes).to_bytes(4, byteorder='big')

        # serialize left
        b += left_bytes
        # serialize right
        b += right_bytes

        self.preserialized = b

    def serialize(self):
        if self.preserialized is None:
            self.preserialize()

        # just push between & shift_correction
        between_bytes = (-self.between).to_bytes(4, byteorder='big')
        shift_correction_bytes = self.shift_correction.to_bytes(4, byteorder='big')
        return between_bytes + shift_correction_bytes + self.preserialized[8:]

    @staticmethod
    def check(b: bytes):
        return len(b) > 16
