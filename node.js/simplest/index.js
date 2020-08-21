
const WebSocket = require('ws')
const server = new WebSocket.Server({port:8888})

server.on('connection', socket => {
	server.on('message', message => {
		for (const client of server.clients)
			if (client.readyState == WebSocket.OPEN)
				client.send(message)
	})
})

