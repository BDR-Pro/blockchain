import asyncio
import websockets

async def test_ws():
    uri = "ws://localhost:8080"
    async with websockets.connect(uri) as websocket:
        # Sending a "ping" message
        await websocket.send("ping")
        print("> ping")

        # Waiting for a response
        response = await websocket.recv()
        print(f"< {response}")

# Run the asynchronous function
asyncio.run(test_ws())

