package main

import (
	"fmt"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/t10471/serializers/fbs/P2P"
)

func main() {
	builder := flatbuffers.NewBuilder(0)
	P2P.MessageStart(builder)
	m := P2P.MessageEnd(builder)
	builder.Finish(m)
	fmt.Printf("Serialize: %v\n", builder.Bytes)

	msg := P2P.GetRootAsMessage(builder.Bytes, builder.Head())
	fmt.Printf("Deserialize: %v\n", msg)
	fmt.Printf("Type: %v\n", P2P.EnumNamesMessageType[msg.Type()])

	builder = flatbuffers.NewBuilder(0)
	P2P.MessageStart(builder)
	P2P.MessageAddType(builder, P2P.MessageTypeHandshake)
	m = P2P.MessageEnd(builder)
	builder.Finish(m)

	msg = P2P.GetRootAsMessage(builder.Bytes, builder.Head())
	fmt.Printf("Deserialize: %v\n", msg)
	fmt.Printf("Type: %v\n", P2P.EnumNamesMessageType[msg.Type()])

	builder = flatbuffers.NewBuilder(0)
	P2P.MessageStart(builder)
	P2P.MessageAddType(builder, P2P.MessageTypeHandshake)
	P2P.MessageAddStatus(builder, P2P.StatusNG)
	//P2P.MessageAddType(builder, P2P.StatusNG)
	m = P2P.MessageEnd(builder)
	builder.Finish(m)

	msg = P2P.GetRootAsMessage(builder.Bytes, builder.Head())
	fmt.Printf("Deserialize: %v\n", msg)
	fmt.Printf("Type: %v\n", P2P.EnumNamesMessageType[msg.Type()])

	builder = flatbuffers.NewBuilder(0)
	P2P.MessageStart(builder)
	P2P.MessageAddType(builder, P2P.MessageTypeBye)
	m = P2P.MessageEnd(builder)
	builder.Finish(m)

	msg = P2P.GetRootAsMessage(builder.Bytes, builder.Head())
	fmt.Printf("Deserialize: %v\n", msg)
	fmt.Printf("Type: %v\n", P2P.EnumNamesMessageType[msg.Type()])
}
