import os
import threading
import time

import zmq
import cv2
from server import Message
from readchar import readchar


class InputHandler(threading.Thread):
    def __init__(self):
        super().__init__()
        self.daemon = True
        self.distance = -10000
        self.shift_correction = 0

    def run(self):
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


left = cv2.imread("crates/vr_renderer/local-images/example_L.png")
right = cv2.imread("crates/vr_renderer/local-images/example_R.png")

context = zmq.Context()
socket = context.socket(zmq.REQ)
socket.connect("tcp://localhost:3459")

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

    time_network_before = time.time_ns()
    socket.send(serial)
    socket.recv()
    time_network_after = time.time_ns()

    idx += 1
    if idx % 10 == 0:
        idx = 0
        print(f"Builder: {(time_builder_after - time_builder_before) / 1_000_000:.2f}ms, Network: {(time_network_after - time_network_before) / 1_000_000:.2f}ms")
