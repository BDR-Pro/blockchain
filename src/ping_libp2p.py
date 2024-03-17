import socket
import time

def tcp_ping(host, port, timeout=1):
    """
    Attempt to establish a TCP connection to the specified host and port.
    If the connection is successful, the service is considered up.

    :param host: The hostname or IP address of the target.
    :param port: The target port to connect to.
    :param timeout: Connection timeout in seconds.
    :return: A tuple containing a boolean indicating success and the ping time in milliseconds.
    """
    try:
        start_time = time.time()
        # Create a socket object
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
            # Set the timeout for the socket
            sock.settimeout(timeout)
            # Attempt to connect to the host and port
            sock.connect((host, port))
            end_time = time.time()
        return True, (end_time - start_time) * 1000  # Return True and the ping time in ms
    except socket.error as e:
        print(f"Connection to {host}:{port} failed: {e}")
        return False, None

#/ip4/172.19.240.1/tcp/8000"
host = "172.19.240.1"
port = 8000
success, ping_time = tcp_ping(host, port)
if success:
    print(f"TCP ping to {host}:{port} succeeded in {ping_time:.2f} ms")
else:
    print(f"TCP ping to {host}:{port} failed.")
