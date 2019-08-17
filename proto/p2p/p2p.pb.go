// Code generated by protoc-gen-go. DO NOT EDIT.
// source: p2p.proto

package p2p

import (
	fmt "fmt"
	proto "github.com/golang/protobuf/proto"
	math "math"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

type Message_MessageType int32

const (
	Message_Handshake       Message_MessageType = 0
	Message_SendTransaction Message_MessageType = 1
	Message_SendBlock       Message_MessageType = 2
	Message_Bye             Message_MessageType = 3
)

var Message_MessageType_name = map[int32]string{
	0: "Handshake",
	1: "SendTransaction",
	2: "SendBlock",
	3: "Bye",
}

var Message_MessageType_value = map[string]int32{
	"Handshake":       0,
	"SendTransaction": 1,
	"SendBlock":       2,
	"Bye":             3,
}

func (x Message_MessageType) String() string {
	return proto.EnumName(Message_MessageType_name, int32(x))
}

func (Message_MessageType) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_e7fdddb109e6467a, []int{0, 0}
}

type Message_Status int32

const (
	Message_OK Message_Status = 0
	Message_NG Message_Status = 1
)

var Message_Status_name = map[int32]string{
	0: "OK",
	1: "NG",
}

var Message_Status_value = map[string]int32{
	"OK": 0,
	"NG": 1,
}

func (x Message_Status) String() string {
	return proto.EnumName(Message_Status_name, int32(x))
}

func (Message_Status) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_e7fdddb109e6467a, []int{0, 1}
}

type Message struct {
	Type                 Message_MessageType `protobuf:"varint,1,opt,name=type,proto3,enum=Message_MessageType" json:"type,omitempty"`
	Status               Message_Status      `protobuf:"varint,2,opt,name=status,proto3,enum=Message_Status" json:"status,omitempty"`
	Payload              []byte              `protobuf:"bytes,3,opt,name=payload,proto3" json:"payload,omitempty"`
	XXX_NoUnkeyedLiteral struct{}            `json:"-"`
	XXX_unrecognized     []byte              `json:"-"`
	XXX_sizecache        int32               `json:"-"`
}

func (m *Message) Reset()         { *m = Message{} }
func (m *Message) String() string { return proto.CompactTextString(m) }
func (*Message) ProtoMessage()    {}
func (*Message) Descriptor() ([]byte, []int) {
	return fileDescriptor_e7fdddb109e6467a, []int{0}
}

func (m *Message) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Message.Unmarshal(m, b)
}
func (m *Message) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Message.Marshal(b, m, deterministic)
}
func (m *Message) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Message.Merge(m, src)
}
func (m *Message) XXX_Size() int {
	return xxx_messageInfo_Message.Size(m)
}
func (m *Message) XXX_DiscardUnknown() {
	xxx_messageInfo_Message.DiscardUnknown(m)
}

var xxx_messageInfo_Message proto.InternalMessageInfo

func (m *Message) GetType() Message_MessageType {
	if m != nil {
		return m.Type
	}
	return Message_Handshake
}

func (m *Message) GetStatus() Message_Status {
	if m != nil {
		return m.Status
	}
	return Message_OK
}

func (m *Message) GetPayload() []byte {
	if m != nil {
		return m.Payload
	}
	return nil
}

func init() {
	proto.RegisterEnum("Message_MessageType", Message_MessageType_name, Message_MessageType_value)
	proto.RegisterEnum("Message_Status", Message_Status_name, Message_Status_value)
	proto.RegisterType((*Message)(nil), "Message")
}

func init() { proto.RegisterFile("p2p.proto", fileDescriptor_e7fdddb109e6467a) }

var fileDescriptor_e7fdddb109e6467a = []byte{
	// 204 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x4c, 0x8f, 0x4d, 0x4e, 0x86, 0x30,
	0x10, 0x86, 0x29, 0x98, 0x12, 0xc6, 0x1f, 0x9a, 0xea, 0xa2, 0x4b, 0xc2, 0x46, 0x56, 0x2c, 0xf0,
	0x06, 0x6c, 0xd4, 0x18, 0x35, 0x01, 0x2e, 0x30, 0x42, 0xa3, 0x06, 0xd2, 0x36, 0xb4, 0x2e, 0x7a,
	0x56, 0x2f, 0x63, 0xa8, 0xf0, 0xe5, 0x5b, 0xbd, 0x99, 0x67, 0x9e, 0xcc, 0xe4, 0x85, 0xcc, 0x34,
	0xa6, 0x36, 0xab, 0x76, 0xba, 0xfc, 0x25, 0x90, 0xbe, 0x4a, 0x6b, 0xf1, 0x53, 0xf2, 0x0a, 0x2e,
	0x9c, 0x37, 0x52, 0x90, 0x82, 0x54, 0x37, 0xcd, 0x5d, 0xbd, 0xf3, 0x23, 0x07, 0x6f, 0x64, 0x17,
	0x0c, 0x7e, 0x0f, 0xd4, 0x3a, 0x74, 0x3f, 0x56, 0xc4, 0xc1, 0xcd, 0x4f, 0x6e, 0x1f, 0x70, 0xb7,
	0xaf, 0xb9, 0x80, 0xd4, 0xa0, 0x5f, 0x34, 0x4e, 0x22, 0x29, 0x48, 0x75, 0xd5, 0x1d, 0x63, 0xf9,
	0x0c, 0x97, 0x67, 0x77, 0xf9, 0x35, 0x64, 0x4f, 0xa8, 0x26, 0xfb, 0x85, 0xb3, 0x64, 0x11, 0xbf,
	0x85, 0xbc, 0x97, 0x6a, 0x1a, 0x56, 0x54, 0x16, 0x47, 0xf7, 0xad, 0x15, 0x23, 0x9b, 0xb3, 0xc1,
	0x76, 0xd1, 0xe3, 0xcc, 0x62, 0x9e, 0x42, 0xd2, 0x7a, 0xc9, 0x92, 0x52, 0x00, 0xfd, 0x7f, 0xcb,
	0x29, 0xc4, 0xef, 0x2f, 0x2c, 0xda, 0xf2, 0xed, 0x91, 0x91, 0x0f, 0x1a, 0x4a, 0x3e, 0xfc, 0x05,
	0x00, 0x00, 0xff, 0xff, 0x31, 0x1e, 0xf8, 0xf4, 0xf1, 0x00, 0x00, 0x00,
}
