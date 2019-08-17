// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package P2P

type MessageType = int8
const (
	MessageTypeHandshake MessageType = 0
	MessageTypeSendTransaction MessageType = 1
	MessageTypeSendBlock MessageType = 2
	MessageTypeBye MessageType = 3
)

var EnumNamesMessageType = map[MessageType]string{
	MessageTypeHandshake:"Handshake",
	MessageTypeSendTransaction:"SendTransaction",
	MessageTypeSendBlock:"SendBlock",
	MessageTypeBye:"Bye",
}
