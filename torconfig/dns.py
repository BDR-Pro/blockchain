import asyncio
import websockets
import aiohttp
from aiohttp_socks import ProxyConnector

# Define the port for the WebSocket server
port = 8888

# Function to ping your own server
async def ping_my_self():
    try:
        with open("hostname") as file:
            hostname = file.read().strip()
        print(f"Pinging myself: {hostname}")
        await ping(f"ws://{hostname}:{port}")  
    except Exception as e:
        print(f"Failed to ping myself: {e}")
        return False

# The ping function, which establishes a WebSocket connection and sends a message
async def ping(uri, proxy="socks5://localhost:9050"):
    connector = ProxyConnector.from_url(proxy)
    async with aiohttp.ClientSession(connector=connector) as session:
        async with session.ws_connect(uri) as ws:
            await ws.send_str("ping")
            msg = await ws.receive()
            print(f"Received response: {msg.data}")

# Function to check if the current connection is using Tor
async def is_it_tor(session):
    try:
        async with session.get("https://check.torproject.org/api/ip") as response:
            data = await response.json()
            return data.get('IsTor')  # Adjust depending on actual response format
    except Exception as e:
        print(f"Error checking Tor status: {e}")
        return False

# The main WebSocket echo function
async def echo(websocket, path):
    async for message in websocket:
        print(f"Received message: {message} from {websocket.remote_address}")
        if message == "ping":
            try:
                with open("dns.txt", "r") as file:
                    last_line = file.readlines()[-1].strip()
                await websocket.send(last_line)
            except Exception as e:
                print(f"Failed to send last line from dns.txt: {e}")
                await websocket.send("Error: Could not retrieve data.")

# Main function to start the server and handle other asynchronous tasks
async def main():
    # Set up the proxy connector for Tor
    proxy = "socks5://localhost:9050"
    connector = ProxyConnector.from_url(proxy)
    
    # Check if connected via Tor
    async with aiohttp.ClientSession(connector=connector) as session:
        is_tor = await is_it_tor(session)
        print(f"Connected to Tor: {is_tor}")

    # Set up and start the WebSocket server
    server = await websockets.serve(echo, "0.0.0.0", port)
    print(f"DNS Server started at port {port}")

    # Optionally, you can schedule the ping_my_self function as a background task
    asyncio.create_task(ping_my_self())


    # This will keep the coroutine running indefinitely
    await asyncio.Future() 

# Run the main function
asyncio.run(main())
