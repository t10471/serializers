package main

import (
	"fmt"

	"github.com/golang/protobuf/proto"
	"github.com/t10471/serializers/proto/p2p"
)

func main() {
	m := &p2p.Message{}
	buf, _ := proto.Marshal(m)
	fmt.Printf("Serialize: %v\n", buf)
	msg := &p2p.Message{}
	_ = proto.Unmarshal(buf, msg)
	fmt.Printf("Deserialize: %v\n", msg)
	fmt.Printf("Type: %v\n", msg.Type)

	m = &p2p.Message{Type: p2p.Message_Handshake}
	buf, _ = proto.Marshal(m)
	fmt.Printf("Serialize: %v\n", buf)
	msg = &p2p.Message{}
	_ = proto.Unmarshal(buf, msg)
	fmt.Printf("Deserialize: %v\n", msg)
	fmt.Printf("Type: %v\n", msg.Type)

	m = &p2p.Message{Type: p2p.Message_Handshake, Status: p2p.Message_NG}
	buf, _ = proto.Marshal(m)
	fmt.Printf("Serialize: %v\n", buf)
	msg = &p2p.Message{}
	_ = proto.Unmarshal(buf, msg)
	fmt.Printf("Deserialize: %v\n", msg)
	fmt.Printf("Type: %v\n", msg.Type)

	m = &p2p.Message{Type: p2p.Message_Bye}
	buf, _ = proto.Marshal(m)
	fmt.Printf("Serialize: %v\n", buf)
	msg = &p2p.Message{}
	_ = proto.Unmarshal(buf, msg)
	fmt.Printf("Deserialize: %v\n", msg)
	fmt.Printf("Type: %v\n", msg.Type)
}
