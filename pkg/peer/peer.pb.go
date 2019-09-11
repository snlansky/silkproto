// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: peer.proto

package proto

import (
	"fmt"
	"math"

	proto "github.com/golang/protobuf/proto"

	common "github.com/snlansky/silkproto/pkg/common"

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

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// Client API for Endorser service

type EndorserClient interface {
	ProcessProposal(ctx context.Context, in *common.SignedProposal, opts ...grpc.CallOption) (*common.ProposalResponse, error)
}

type endorserClient struct {
	cc *grpc.ClientConn
}

func NewEndorserClient(cc *grpc.ClientConn) EndorserClient {
	return &endorserClient{cc}
}

func (c *endorserClient) ProcessProposal(ctx context.Context, in *common.SignedProposal, opts ...grpc.CallOption) (*common.ProposalResponse, error) {
	out := new(common.ProposalResponse)
	err := c.cc.Invoke(ctx, "/proto.Endorser/ProcessProposal", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// Server API for Endorser service

type EndorserServer interface {
	ProcessProposal(context.Context, *common.SignedProposal) (*common.ProposalResponse, error)
}

func RegisterEndorserServer(s *grpc.Server, srv EndorserServer) {
	s.RegisterService(&_Endorser_serviceDesc, srv)
}

func _Endorser_ProcessProposal_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(common.SignedProposal)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(EndorserServer).ProcessProposal(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/proto.Endorser/ProcessProposal",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(EndorserServer).ProcessProposal(ctx, req.(*common.SignedProposal))
	}
	return interceptor(ctx, in, info, handler)
}

var _Endorser_serviceDesc = grpc.ServiceDesc{
	ServiceName: "proto.Endorser",
	HandlerType: (*EndorserServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "ProcessProposal",
			Handler:    _Endorser_ProcessProposal_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "peer.proto",
}

func init() { proto.RegisterFile("peer.proto", fileDescriptor_peer_a381a539b84b01e8) }

var fileDescriptor_peer_a381a539b84b01e8 = []byte{
	// 133 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0xe2, 0x2a, 0x48, 0x4d, 0x2d,
	0xd2, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0x62, 0x05, 0x53, 0x52, 0x3c, 0xc9, 0xf9, 0xb9, 0xb9,
	0xf9, 0x79, 0x10, 0x41, 0x23, 0x7f, 0x2e, 0x0e, 0xd7, 0xbc, 0x94, 0xfc, 0xa2, 0xe2, 0xd4, 0x22,
	0x21, 0x67, 0x2e, 0xfe, 0x80, 0xa2, 0xfc, 0xe4, 0xd4, 0xe2, 0xe2, 0x80, 0xa2, 0xfc, 0x82, 0xfc,
	0xe2, 0xc4, 0x1c, 0x21, 0x51, 0x88, 0x32, 0xbd, 0xe0, 0xcc, 0xf4, 0xbc, 0xd4, 0x14, 0x98, 0xb0,
	0x94, 0x38, 0x54, 0x18, 0x26, 0x10, 0x94, 0x5a, 0x5c, 0x90, 0x9f, 0x57, 0x9c, 0xaa, 0xc4, 0xe0,
	0x24, 0x70, 0xe2, 0x91, 0x1c, 0xe3, 0x85, 0x47, 0x72, 0x8c, 0x0f, 0x1e, 0xc9, 0x31, 0xce, 0x78,
	0x2c, 0xc7, 0x90, 0xc4, 0x06, 0x56, 0x6b, 0x0c, 0x08, 0x00, 0x00, 0xff, 0xff, 0x03, 0xb6, 0x0f,
	0xd0, 0x8c, 0x00, 0x00, 0x00,
}
