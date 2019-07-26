// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: consensus.proto

package proto

import (
	"fmt"
	"io"
	"math"

	proto "github.com/golang/protobuf/proto"

	common "github.com/pingcap/kvproto/pkg/common"

	context "golang.org/x/net/context"

	grpc "google.golang.org/grpc"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion2 // please upgrade the proto package

type VariableType int32

const (
	VariableType_RaftProposeType VariableType = 0
	VariableType_RaftRewType     VariableType = 1
)

var VariableType_name = map[int32]string{
	0: "RaftProposeType",
	1: "RaftRewType",
}
var VariableType_value = map[string]int32{
	"RaftProposeType": 0,
	"RaftRewType":     1,
}

func (x VariableType) String() string {
	return proto.EnumName(VariableType_name, int32(x))
}
func (VariableType) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_consensus_44e750be9ef2ab83, []int{0}
}

type VariableMessage struct {
	Vt                   VariableType `protobuf:"varint,1,opt,name=vt,proto3,enum=proto.VariableType" json:"vt,omitempty"`
	Payload              []byte       `protobuf:"bytes,2,opt,name=payload,proto3" json:"payload,omitempty"`
	XXX_NoUnkeyedLiteral struct{}     `json:"-"`
	XXX_unrecognized     []byte       `json:"-"`
	XXX_sizecache        int32        `json:"-"`
}

func (m *VariableMessage) Reset()         { *m = VariableMessage{} }
func (m *VariableMessage) String() string { return proto.CompactTextString(m) }
func (*VariableMessage) ProtoMessage()    {}
func (*VariableMessage) Descriptor() ([]byte, []int) {
	return fileDescriptor_consensus_44e750be9ef2ab83, []int{0}
}
func (m *VariableMessage) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *VariableMessage) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_VariableMessage.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *VariableMessage) XXX_Merge(src proto.Message) {
	xxx_messageInfo_VariableMessage.Merge(dst, src)
}
func (m *VariableMessage) XXX_Size() int {
	return m.Size()
}
func (m *VariableMessage) XXX_DiscardUnknown() {
	xxx_messageInfo_VariableMessage.DiscardUnknown(m)
}

var xxx_messageInfo_VariableMessage proto.InternalMessageInfo

func (m *VariableMessage) GetVt() VariableType {
	if m != nil {
		return m.Vt
	}
	return VariableType_RaftProposeType
}

func (m *VariableMessage) GetPayload() []byte {
	if m != nil {
		return m.Payload
	}
	return nil
}

type BroadcastResponse struct {
	// Status code, which may be used to programatically respond to success/failure
	Status common.Status `protobuf:"varint,1,opt,name=status,proto3,enum=proto.Status" json:"status,omitempty"`
	// Info string which may contain additional information about the status returned
	Info                 string   `protobuf:"bytes,2,opt,name=info,proto3" json:"info,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *BroadcastResponse) Reset()         { *m = BroadcastResponse{} }
func (m *BroadcastResponse) String() string { return proto.CompactTextString(m) }
func (*BroadcastResponse) ProtoMessage()    {}
func (*BroadcastResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_consensus_44e750be9ef2ab83, []int{1}
}
func (m *BroadcastResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *BroadcastResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_BroadcastResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *BroadcastResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_BroadcastResponse.Merge(dst, src)
}
func (m *BroadcastResponse) XXX_Size() int {
	return m.Size()
}
func (m *BroadcastResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_BroadcastResponse.DiscardUnknown(m)
}

var xxx_messageInfo_BroadcastResponse proto.InternalMessageInfo

func (m *BroadcastResponse) GetStatus() common.Status {
	if m != nil {
		return m.Status
	}
	return common.Status_UNKNOWN
}

func (m *BroadcastResponse) GetInfo() string {
	if m != nil {
		return m.Info
	}
	return ""
}

func init() {
	proto.RegisterType((*VariableMessage)(nil), "proto.VariableMessage")
	proto.RegisterType((*BroadcastResponse)(nil), "proto.BroadcastResponse")
	proto.RegisterEnum("proto.VariableType", VariableType_name, VariableType_value)
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// Client API for AtomicBroadcast service

type AtomicBroadcastClient interface {
	Broadcast(ctx context.Context, opts ...grpc.CallOption) (AtomicBroadcast_BroadcastClient, error)
}

type atomicBroadcastClient struct {
	cc *grpc.ClientConn
}

func NewAtomicBroadcastClient(cc *grpc.ClientConn) AtomicBroadcastClient {
	return &atomicBroadcastClient{cc}
}

func (c *atomicBroadcastClient) Broadcast(ctx context.Context, opts ...grpc.CallOption) (AtomicBroadcast_BroadcastClient, error) {
	stream, err := c.cc.NewStream(ctx, &_AtomicBroadcast_serviceDesc.Streams[0], "/proto.AtomicBroadcast/Broadcast", opts...)
	if err != nil {
		return nil, err
	}
	x := &atomicBroadcastBroadcastClient{stream}
	return x, nil
}

type AtomicBroadcast_BroadcastClient interface {
	Send(*common.ChannelHeader) error
	Recv() (*BroadcastResponse, error)
	grpc.ClientStream
}

type atomicBroadcastBroadcastClient struct {
	grpc.ClientStream
}

func (x *atomicBroadcastBroadcastClient) Send(m *common.ChannelHeader) error {
	return x.ClientStream.SendMsg(m)
}

func (x *atomicBroadcastBroadcastClient) Recv() (*BroadcastResponse, error) {
	m := new(BroadcastResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// Server API for AtomicBroadcast service

type AtomicBroadcastServer interface {
	Broadcast(AtomicBroadcast_BroadcastServer) error
}

func RegisterAtomicBroadcastServer(s *grpc.Server, srv AtomicBroadcastServer) {
	s.RegisterService(&_AtomicBroadcast_serviceDesc, srv)
}

func _AtomicBroadcast_Broadcast_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(AtomicBroadcastServer).Broadcast(&atomicBroadcastBroadcastServer{stream})
}

type AtomicBroadcast_BroadcastServer interface {
	Send(*BroadcastResponse) error
	Recv() (*common.ChannelHeader, error)
	grpc.ServerStream
}

type atomicBroadcastBroadcastServer struct {
	grpc.ServerStream
}

func (x *atomicBroadcastBroadcastServer) Send(m *BroadcastResponse) error {
	return x.ServerStream.SendMsg(m)
}

func (x *atomicBroadcastBroadcastServer) Recv() (*common.ChannelHeader, error) {
	m := new(common.ChannelHeader)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

var _AtomicBroadcast_serviceDesc = grpc.ServiceDesc{
	ServiceName: "proto.AtomicBroadcast",
	HandlerType: (*AtomicBroadcastServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Broadcast",
			Handler:       _AtomicBroadcast_Broadcast_Handler,
			ServerStreams: true,
			ClientStreams: true,
		},
	},
	Metadata: "consensus.proto",
}

func (m *VariableMessage) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *VariableMessage) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Vt != 0 {
		dAtA[i] = 0x8
		i++
		i = encodeVarintConsensus(dAtA, i, uint64(m.Vt))
	}
	if len(m.Payload) > 0 {
		dAtA[i] = 0x12
		i++
		i = encodeVarintConsensus(dAtA, i, uint64(len(m.Payload)))
		i += copy(dAtA[i:], m.Payload)
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *BroadcastResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *BroadcastResponse) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Status != 0 {
		dAtA[i] = 0x8
		i++
		i = encodeVarintConsensus(dAtA, i, uint64(m.Status))
	}
	if len(m.Info) > 0 {
		dAtA[i] = 0x12
		i++
		i = encodeVarintConsensus(dAtA, i, uint64(len(m.Info)))
		i += copy(dAtA[i:], m.Info)
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func encodeVarintConsensus(dAtA []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return offset + 1
}
func (m *VariableMessage) Size() (n int) {
	var l int
	_ = l
	if m.Vt != 0 {
		n += 1 + sovConsensus(uint64(m.Vt))
	}
	l = len(m.Payload)
	if l > 0 {
		n += 1 + l + sovConsensus(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *BroadcastResponse) Size() (n int) {
	var l int
	_ = l
	if m.Status != 0 {
		n += 1 + sovConsensus(uint64(m.Status))
	}
	l = len(m.Info)
	if l > 0 {
		n += 1 + l + sovConsensus(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovConsensus(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozConsensus(x uint64) (n int) {
	return sovConsensus(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *VariableMessage) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowConsensus
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: VariableMessage: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: VariableMessage: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Vt", wireType)
			}
			m.Vt = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowConsensus
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Vt |= (VariableType(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Payload", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowConsensus
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthConsensus
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Payload = append(m.Payload[:0], dAtA[iNdEx:postIndex]...)
			if m.Payload == nil {
				m.Payload = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipConsensus(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthConsensus
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, dAtA[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *BroadcastResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowConsensus
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: BroadcastResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: BroadcastResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Status", wireType)
			}
			m.Status = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowConsensus
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Status |= (common.Status(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Info", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowConsensus
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthConsensus
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Info = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipConsensus(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthConsensus
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, dAtA[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipConsensus(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowConsensus
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		wireType := int(wire & 0x7)
		switch wireType {
		case 0:
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowConsensus
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
			return iNdEx, nil
		case 1:
			iNdEx += 8
			return iNdEx, nil
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowConsensus
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			iNdEx += length
			if length < 0 {
				return 0, ErrInvalidLengthConsensus
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowConsensus
					}
					if iNdEx >= l {
						return 0, io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					innerWire |= (uint64(b) & 0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				innerWireType := int(innerWire & 0x7)
				if innerWireType == 4 {
					break
				}
				next, err := skipConsensus(dAtA[start:])
				if err != nil {
					return 0, err
				}
				iNdEx = start + next
			}
			return iNdEx, nil
		case 4:
			return iNdEx, nil
		case 5:
			iNdEx += 4
			return iNdEx, nil
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
	}
	panic("unreachable")
}

var (
	ErrInvalidLengthConsensus = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowConsensus   = fmt.Errorf("proto: integer overflow")
)

func init() { proto.RegisterFile("consensus.proto", fileDescriptor_consensus_44e750be9ef2ab83) }

var fileDescriptor_consensus_44e750be9ef2ab83 = []byte{
	// 278 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x64, 0x8f, 0x41, 0x4e, 0xf3, 0x30,
	0x10, 0x85, 0xe3, 0xe8, 0xff, 0x8b, 0x3a, 0x04, 0x52, 0x5c, 0x16, 0x51, 0x17, 0x51, 0x15, 0x84,
	0x14, 0xb1, 0xa8, 0x50, 0xe1, 0x02, 0x2d, 0x1b, 0x36, 0xa0, 0xca, 0x54, 0xec, 0xa7, 0x89, 0x0b,
	0x91, 0x12, 0x4f, 0x14, 0xbb, 0x45, 0xbd, 0x09, 0x47, 0x62, 0xc9, 0x11, 0x50, 0xb8, 0x08, 0xaa,
	0xeb, 0x02, 0x12, 0x2b, 0xfb, 0x7d, 0x7e, 0x7e, 0xf3, 0x06, 0xc2, 0x8c, 0x94, 0x96, 0x4a, 0xaf,
	0xf4, 0xa8, 0x6e, 0xc8, 0x10, 0xff, 0x6f, 0x8f, 0x41, 0x90, 0x51, 0x55, 0x91, 0xda, 0xc1, 0x64,
	0x06, 0xe1, 0x23, 0x36, 0x05, 0x2e, 0x4a, 0x79, 0x27, 0xb5, 0xc6, 0x27, 0xc9, 0xcf, 0xc0, 0x5f,
	0x9b, 0x88, 0x0d, 0x59, 0x7a, 0x3c, 0xee, 0xef, 0x6c, 0xa3, 0xbd, 0x67, 0xbe, 0xa9, 0xa5, 0xf0,
	0xd7, 0x86, 0x47, 0x70, 0x50, 0xe3, 0xa6, 0x24, 0xcc, 0x23, 0x7f, 0xc8, 0xd2, 0x40, 0xec, 0x65,
	0x72, 0x0f, 0x27, 0xd3, 0x86, 0x30, 0xcf, 0x50, 0x1b, 0x21, 0x75, 0xbd, 0x6d, 0xc1, 0xcf, 0xa1,
	0xa3, 0x0d, 0x9a, 0x95, 0x76, 0xb9, 0x47, 0x2e, 0xf7, 0xc1, 0x42, 0xe1, 0x1e, 0x39, 0x87, 0x7f,
	0x85, 0x5a, 0x92, 0x8d, 0xec, 0x0a, 0x7b, 0xbf, 0xb8, 0x86, 0xe0, 0xf7, 0x74, 0xde, 0x87, 0x50,
	0xe0, 0xd2, 0xcc, 0x1a, 0xaa, 0x49, 0x5b, 0xd4, 0xf3, 0x78, 0x08, 0x87, 0x5b, 0x28, 0xe4, 0x8b,
	0x05, 0x6c, 0x3c, 0x87, 0x70, 0x62, 0xa8, 0x2a, 0xb2, 0xef, 0x2e, 0x7c, 0x02, 0xdd, 0x1f, 0x71,
	0xea, 0x0a, 0xdc, 0x3c, 0xa3, 0x52, 0xb2, 0xbc, 0x95, 0x98, 0xcb, 0x66, 0x10, 0x39, 0xfa, 0x67,
	0x81, 0xc4, 0x4b, 0xd9, 0x25, 0x9b, 0xf6, 0xde, 0xda, 0x98, 0xbd, 0xb7, 0x31, 0xfb, 0x68, 0x63,
	0xf6, 0xfa, 0x19, 0x7b, 0x8b, 0x8e, 0xfd, 0x70, 0xf5, 0x15, 0x00, 0x00, 0xff, 0xff, 0x88, 0x77,
	0x74, 0x6a, 0x6e, 0x01, 0x00, 0x00,
}
