// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: peer.proto

package proto

import (
	"fmt"
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

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// Client API for Endorser service

type EndorserClient interface {
	ProcessProposal(ctx context.Context, in *common.Proposal, opts ...grpc.CallOption) (*common.ProposalResponse, error)
}

type endorserClient struct {
	cc *grpc.ClientConn
}

func NewEndorserClient(cc *grpc.ClientConn) EndorserClient {
	return &endorserClient{cc}
}

func (c *endorserClient) ProcessProposal(ctx context.Context, in *common.Proposal, opts ...grpc.CallOption) (*common.ProposalResponse, error) {
	out := new(common.ProposalResponse)
	err := c.cc.Invoke(ctx, "/proto.Endorser/ProcessProposal", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// Server API for Endorser service

type EndorserServer interface {
	ProcessProposal(context.Context, *common.Proposal) (*common.ProposalResponse, error)
}

func RegisterEndorserServer(s *grpc.Server, srv EndorserServer) {
	s.RegisterService(&_Endorser_serviceDesc, srv)
}

func _Endorser_ProcessProposal_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(common.Proposal)
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
		return srv.(EndorserServer).ProcessProposal(ctx, req.(*common.Proposal))
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

func init() { proto.RegisterFile("peer.proto", fileDescriptor_front_ad43c9b69d479694) }

var fileDescriptor_front_ad43c9b69d479694 = []byte{
	// 126 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0xe2, 0x4e, 0x2b, 0xca, 0xcf,
	0x2b, 0xd1, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0x62, 0x05, 0x53, 0x52, 0x3c, 0xc9, 0xf9, 0xb9,
	0xb9, 0xf9, 0x79, 0x10, 0x41, 0x23, 0x4f, 0x2e, 0x0e, 0xd7, 0xbc, 0x94, 0xfc, 0xa2, 0xe2, 0xd4,
	0x22, 0x21, 0x5b, 0x2e, 0xfe, 0x80, 0xa2, 0xfc, 0xe4, 0xd4, 0xe2, 0xe2, 0x80, 0xa2, 0xfc, 0x82,
	0xfc, 0xe2, 0xc4, 0x1c, 0x21, 0x7e, 0x88, 0x32, 0x3d, 0x98, 0x80, 0x94, 0x38, 0x9a, 0x40, 0x50,
	0x6a, 0x71, 0x41, 0x7e, 0x5e, 0x71, 0xaa, 0x12, 0x83, 0x93, 0xc0, 0x89, 0x47, 0x72, 0x8c, 0x17,
	0x1e, 0xc9, 0x31, 0x3e, 0x78, 0x24, 0xc7, 0x38, 0xe3, 0xb1, 0x1c, 0x43, 0x12, 0x1b, 0x58, 0xad,
	0x31, 0x20, 0x00, 0x00, 0xff, 0xff, 0xe6, 0x27, 0xc6, 0x03, 0x87, 0x00, 0x00, 0x00,
}
