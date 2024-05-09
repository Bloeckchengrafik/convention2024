from time import time_ns

import zmq

from server import Message
from server.display import DisplaySystem

context = zmq.Context()
socket = context.socket(zmq.REP)
socket.bind("tcp://*:3459")

msg = Message()
display_system = DisplaySystem()
display_system.start()

idx = 0

while True:
    #  Wait for next request from client
    message = socket.recv()

    if not Message.check(message):
        print("Invalid message")
        socket.send(b"Invalid")
        continue

    time_before = time_ns()
    inst_time = msg.read(message)
    time_after = time_ns()
    msg.postprocess()

    socket.send(b"Ok")

    display_system.set_cv_data(msg.left, msg.right, msg.between)

    idx += 1
    if idx % 10 == 0:
        idx = 0
        print(f"read time: {(time_after - time_before) / 1_000_000}ms; inst time: {inst_time/1_000_000}ms")
