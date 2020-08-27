from aiohttp import web

class Broadcast:
    _wss = []

    def __init__(self, ws):
        self._ws = ws
        Broadcast._wss.append(ws)

    def __del__(self):
        Broadcast._wss.remove(self._ws)

    @staticmethod
    async def send(data):
        for ws in Broadcast._wss:
            await ws.send_str(data)


async def websocket_handler(request):
    ws = web.WebSocketResponse()
    await ws.prepare(request)
    broadcast = Broadcast(ws)
    async for msg in ws:
        await broadcast.send(msg.data)
    print('websocket connection closed')
    return ws


app = web.Application()
app.add_routes([web.get('/', websocket_handler)])
web.run_app(app, port=8888, host="127.0.0.1")
