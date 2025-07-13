# Simple script to send RESP commands to a Redis server
# Easy tests to use before running the full test suite


import socket
from time import sleep

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

    # RESP for: SET key value with expiration PX TTL
    resp_set = b"*5\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n$2\r\nPX\r\n$4\r\n1000\r\n"
    response = send_command(resp_set)

    print("SET response:", response)  # Should print: +OK\r\n

    # RESP for: GET key
    resp_get = b"*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n"
    response = send_command(resp_get)

    print("GET response:", response)  # Should print: $5\r\nvalue\r\n

    sleep(2)  # Wait for 1 second to ensure the TTL is respected

    # RESP for: GET key after TTL
    response = send_command(resp_get)
    print("GET response after TTL:", response)  # Should print: $-1\r\n (nil)

