import socket
from time import time_ns

from server import Message
from server.display import DisplaySystem

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind(("", 3459))
s.listen(5)
print(f"Server bound to tcp://*:3459")

msg = Message()
display_system = DisplaySystem()
display_system.start()

idx = 0

while True:
    (client, addr) = s.accept()
    print(f"**CLIENT ONLINE: {addr}, BLOCKING SERVER**")

    while True:
        #  Wait for next request from client
        print("Waiting for message length")
        msglen_bytes = client.recv(4)

        # check if connection is closed
        if not msglen_bytes:
            break

        msglen = int.from_bytes(msglen_bytes, "big")

        message = bytearray()

        # chunk into 160000 byte chunks
        while len(message) < msglen:
            chunk = client.recv(160000)
            if not chunk:
                break
            message.extend(chunk)

        if len(message) < msglen:
            print(f"Expected {msglen} bytes, got {len(message)} bytes")
            break

        if len(message) > msglen:
            # crop the message
            message = message[:msglen]

        if not Message.check(message):
            print("Invalid message")
            continue

        time_before = time_ns()
        inst_time = msg.read(message)
        time_after = time_ns()
        msg.postprocess()

        display_system.set_cv_data(msg.left, msg.right, msg.between)

        idx += 1
        if idx % 10 == 0:
            idx = 0
        print(f"read time: {(time_after - time_before) / 1_000_000}ms; inst time: {inst_time / 1_000_000}ms")

    client.close()

    print(f"**CLIENT OFFLINE: {addr}, UNBLOCKING SERVER**")
    display_system.disconnected()
