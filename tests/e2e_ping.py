# Simple script to send RESP commands to a Redis server
# Easy tests to use before running the full test suite


import socket

def send_command(cmd):
    with socket.create_connection(("127.0.0.1", 6379)) as sock:
        sock.sendall(cmd)
        return sock.recv(1024)

if __name__ == "__main__":
    # RESP for: PING
    resp_ping = b"*1\r\n$4\r\nPING\r\n"
    response = send_command(resp_ping)
    print("PING response:", response)  # Should print: +PONG\r\n

    # RESP for: ECHO "hello"
    resp_echo = b"*2\r\n$4\r\nECHO\r\n$5\r\nhello\r\n"
    response = send_command(resp_echo)
    print("ECHO response:", response)  # Should print: $5\r\nhello\r\n

