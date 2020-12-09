package main

import (
	"flag"
	"fmt"
	"io"
	"log"
	"encoding/binary"
	"encoding/json"
	"net"
	"net/http"
	"sync/atomic"
	"time"

	"github.com/gorilla/websocket"
)

type sourceCmd struct {
	Download	string
	MessageSize	int
	MessageCount	int
	Period		int
}

var addr = flag.String("addr", "0.0.0.0:4000", "http service address")

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true
	},
	ReadBufferSize: 16384,
	WriteBufferSize: 16384,
	// EnableCompression: false,
}
var blob = make([]byte, 1000000)

func readLoop(c *websocket.Conn, done *int32) {
    for {
        if _, _, err := c.NextReader(); err != nil {
            c.Close()
            break
        }
    }
    atomic.StoreInt32(done, 1)
}

func readJsonMessage(c *websocket.Conn,  v interface{}) (mt int, err error) {
	mt, r, err := c.NextReader()
	if err != nil {
		return
	}
	d := json.NewDecoder(r)
	err = d.Decode(v)
	return
}

func sink(w http.ResponseWriter, r *http.Request) {
	c, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Print("source: upgrade:", err)
		return
	}
	defer c.Close()
	for {
		got := 0
		mt, r, err := c.NextReader()
		for err == nil {
			n, err := r.Read(blob)
			got += n
			if err == io.EOF {
				err = nil
				break
			}
		}
		if err != nil {
			log.Println("sink: ", err)
			break
		}
		if mt != websocket.BinaryMessage {
			continue
		}
		ms := float64(time.Now().UnixNano()) / 1000000
		t := fmt.Sprintf("{\"timestamp\":%f,\"messagesize\":%d}\n", ms, got)
		c.WriteMessage(websocket.TextMessage, []byte(t))
	}
}

func ipaddress(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	switch r.Method {
	case "OPTIONS":
		return
	case "GET":
		// allowed
	default:
		http.Error(w, "403 Access denied", http.StatusForbidden)
		return
	}
	ip, port, err := net.SplitHostPort(r.RemoteAddr)
	if err != nil {
		ip = "unknown"
		port = "unknown"
	}
	fmt.Fprintf(w, "{\"remoteip\":\"%s\",\"remoteport\":\"%s\"}\n", ip, port)
}

func source(w http.ResponseWriter, r *http.Request) {
	c, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Print("source: upgrade:", err)
		return
	}
	defer c.Close()
	var cmd sourceCmd
	_, err = readJsonMessage(c, &cmd)
	if err != nil {
		log.Print("source: initial command:", err)
		return
	}
	fmt.Printf("CMD: %+v\n", cmd)
	if cmd.MessageSize == 0 {
		cmd.MessageSize = 100000
	}
	if cmd.MessageSize < 125 {
		cmd.MessageSize = 125
	}
	var done int32
	atomic.StoreInt32(&done, 0)
	go readLoop(c, &done)
	for atomic.LoadInt32(&done) == 0 {
		w, err := c.NextWriter(websocket.BinaryMessage);
		if err == nil {
			t := time.Now().UnixNano() / 1000
			err = binary.Write(w, binary.BigEndian, t)
		}
		if err == nil {
			_, err = w.Write(blob[:cmd.MessageSize - 8])
		}
		if err != nil {
			log.Println("source: ", err)
			break
		}
		w.Close()
	}
}

func main() {
	flag.Parse()
	log.SetFlags(0)
	http.HandleFunc("/speedtest/source", source)
	http.HandleFunc("/speedtest/sink", sink)
	http.HandleFunc("/speedtest/ip", ipaddress)
	log.Fatal(http.ListenAndServe(*addr, nil))
}

