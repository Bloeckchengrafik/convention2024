import os
import threading
import time

import socket
import cv2
from server import Message
from readchar import readchar


def img_pre_crop(img):
    size = (int(1280 / 2), int(970 / 2))
    # resize and crop the image
    img_aspect = img.shape[1] / img.shape[0]
    size_aspect = size[0] / size[1]

    if img_aspect > size_aspect:
        # crop width
        new_width = int(img.shape[0] * size_aspect)
        left = (img.shape[1] - new_width) // 2
        img = img[:, left:left + new_width]
    else:
        # crop height
        new_height = int(img.shape[1] / size_aspect)
        top = (img.shape[0] - new_height) // 2
        img = img[top:top + new_height, :]

    # crop to size
    img = cv2.resize(img, size)
    return img


class InputHandler(threading.Thread):
    def __init__(self):
        super().__init__()
        self.daemon = True
        self.distance = -10000
        self.shift_correction = 0

    def run(self):
        print("w/s: distance, a/d: shift correction, q: quit")
        while True:
            command = readchar()
            if command == "w":
                self.distance += 1
                if self.distance > -1:
                    self.distance = -1
            elif command == "s":
                self.distance -= 1
            elif command == "a":
                self.shift_correction -= 1
                if self.shift_correction < 0:
                    self.shift_correction = 0
            elif command == "d":
                self.shift_correction += 1
            elif command == "q":
                os._exit(0)


left = img_pre_crop(cv2.imread("crates/vr_renderer/local-images/example_L.png"))
right = img_pre_crop(cv2.imread("crates/vr_renderer/local-images/example_R.png"))

# show connection errors
connect_to = os.environ.get("TO", "localhost")
print((connect_to, 3459))

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.settimeout(10)
s.connect((connect_to, 3459))
s.settimeout(None)
print("Connected to server")

input_handler = InputHandler()
input_handler.start()
msg = Message()

msg.left = left
msg.right = right

msg.preserialize()

idx = 0

while True:
    time_builder_before = time.time_ns()
    msg.between = input_handler.distance
    msg.shift_correction = input_handler.shift_correction

    serial = msg.serialize()
    time_builder_after = time.time_ns()
    time.sleep(0.01)

    time_network_before = time.time_ns()
    msglen = len(serial).to_bytes(4, "big")
    s.send(msglen)
    s.send(serial)

    time_network_after = time.time_ns()

    idx += 1
    if idx % 10 == 0:
        idx = 0
    print(
        f"Builder: {(time_builder_after - time_builder_before) / 1_000_000:.2f}ms, Network: {(time_network_after - time_network_before) / 1_000_000:.2f}ms")

    # calculate megabits per second
    # 1 byte = 8 bits
    # 1 megabit = 1_000_000 bits
    # 1 megabit = 125_000 bytes
    mb = len(serial) / 125_000
    print(f"Sent {mb:.2f}MB in {((time_network_after - time_network_before) / 1_000_000) / 1000:.2f}s -> {mb / ((time_network_after - time_network_before) / 1_000_000) * 8:.2f}Mbps")
