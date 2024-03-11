import asyncio
import aiohttp
from aiohttp_socks import ProxyConnector

async def is_it_tor(session):
    # Check if the user is connected to Tor
    try:
        async with session.get("https://check.torproject.org/api/ip") as response:
            data = await response.json()
            for key, value in data.items():
                print(f"{key}: {value}")
            return data.get('IsTor')  # Note: Check the exact key from the response
    except Exception as e:
        print(f"Error checking Tor status: {e}")
        return False

async def test_ws():
    uri = "ws://3hdwjjn2kor75ribq7xiws5hzuh4jwg7llinlngrfrpklqstramqrvqd.onion:8888"
    proxy = "socks5://localhost:9050"
    
    connector = ProxyConnector.from_url(proxy)
    
    async with aiohttp.ClientSession(connector=connector) as session:
        is_tor = await is_it_tor(session)
        print(f"Connected to Tor: {is_tor}")
        
        if not is_tor:
            print("You are not connected to Tor.")
            return
        
        async with session.ws_connect(uri) as ws:
            print("Sending 'ping'...")
            await ws.send_str("ping")

            msg = await ws.receive()
            print(f"< {msg.data}")

# Run the asynchronous function
asyncio.run(test_ws())
