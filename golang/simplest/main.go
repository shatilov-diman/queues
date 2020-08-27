package main

import (
	"github.com/gorilla/websocket"
	"log"
	"net/http"
	"sync"
)

type server struct {
	guard    sync.Mutex
	clients  map[*websocket.Conn]bool
	upgrader websocket.Upgrader
}

func (s *server) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	ws, err := s.upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("failed to upgrade connection: %v\n", err)
		return
	}
	defer ws.Close()

	log.Println("client connected")
	defer log.Println("client disconnected")

	s.addSubscriber(ws)
	defer s.removeSubscriber(ws)

	var wg sync.WaitGroup
	wg.Add(1)
	defer wg.Wait()

	var msgC = make(chan *websocket.PreparedMessage, 1000)
	defer close(msgC)

	go func() {
		defer wg.Done()
		for msg := range msgC {
			if err := s.broadcast(msg); err != nil {
				log.Printf("failed to write message: %v\n", err)
				break
			}
		}
	}()

	for {
		msgType, msgData, err := ws.ReadMessage()
		if err != nil {
			log.Printf("failed to read message: %v\n", err)
			break
		}
		msg, err := websocket.NewPreparedMessage(msgType, msgData)
		if err != nil {
			log.Printf("failed to prepare message: %v\n", err)
			break
		}
		msgC <- msg
	}
}

func (s *server) addSubscriber(ws *websocket.Conn) {
	s.guard.Lock()
	defer s.guard.Unlock()
	s.clients[ws] = true
}

func (s *server) removeSubscriber(ws *websocket.Conn) {
	s.guard.Lock()
	defer s.guard.Unlock()
	delete(s.clients, ws)
}

func (s *server) broadcast(msg *websocket.PreparedMessage) error {
	s.guard.Lock()
	defer s.guard.Unlock()
	for ws := range s.clients {
		if err := ws.WritePreparedMessage(msg); err != nil {
			return err
		}
	}
	return nil
}

func main() {
	const addr = ":8888"
	var s = server{
		clients:  make(map[*websocket.Conn]bool),
		upgrader: websocket.Upgrader{},
	}
	http.Handle("/", &s)
	log.Printf("start listening @ %s\n", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}
