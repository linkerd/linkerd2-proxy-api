// Code generated by protoc-gen-go-grpc. DO NOT EDIT.

package tap

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// TapClient is the client API for Tap service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type TapClient interface {
	Observe(ctx context.Context, in *ObserveRequest, opts ...grpc.CallOption) (Tap_ObserveClient, error)
}

type tapClient struct {
	cc grpc.ClientConnInterface
}

func NewTapClient(cc grpc.ClientConnInterface) TapClient {
	return &tapClient{cc}
}

func (c *tapClient) Observe(ctx context.Context, in *ObserveRequest, opts ...grpc.CallOption) (Tap_ObserveClient, error) {
	stream, err := c.cc.NewStream(ctx, &Tap_ServiceDesc.Streams[0], "/io.linkerd.proxy.tap.Tap/Observe", opts...)
	if err != nil {
		return nil, err
	}
	x := &tapObserveClient{stream}
	if err := x.ClientStream.SendMsg(in); err != nil {
		return nil, err
	}
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	return x, nil
}

type Tap_ObserveClient interface {
	Recv() (*TapEvent, error)
	grpc.ClientStream
}

type tapObserveClient struct {
	grpc.ClientStream
}

func (x *tapObserveClient) Recv() (*TapEvent, error) {
	m := new(TapEvent)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// TapServer is the server API for Tap service.
// All implementations must embed UnimplementedTapServer
// for forward compatibility
type TapServer interface {
	Observe(*ObserveRequest, Tap_ObserveServer) error
	mustEmbedUnimplementedTapServer()
}

// UnimplementedTapServer must be embedded to have forward compatible implementations.
type UnimplementedTapServer struct {
}

func (UnimplementedTapServer) Observe(*ObserveRequest, Tap_ObserveServer) error {
	return status.Errorf(codes.Unimplemented, "method Observe not implemented")
}
func (UnimplementedTapServer) mustEmbedUnimplementedTapServer() {}

// UnsafeTapServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to TapServer will
// result in compilation errors.
type UnsafeTapServer interface {
	mustEmbedUnimplementedTapServer()
}

func RegisterTapServer(s grpc.ServiceRegistrar, srv TapServer) {
	s.RegisterService(&Tap_ServiceDesc, srv)
}

func _Tap_Observe_Handler(srv interface{}, stream grpc.ServerStream) error {
	m := new(ObserveRequest)
	if err := stream.RecvMsg(m); err != nil {
		return err
	}
	return srv.(TapServer).Observe(m, &tapObserveServer{stream})
}

type Tap_ObserveServer interface {
	Send(*TapEvent) error
	grpc.ServerStream
}

type tapObserveServer struct {
	grpc.ServerStream
}

func (x *tapObserveServer) Send(m *TapEvent) error {
	return x.ServerStream.SendMsg(m)
}

// Tap_ServiceDesc is the grpc.ServiceDesc for Tap service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Tap_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "io.linkerd.proxy.tap.Tap",
	HandlerType: (*TapServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Observe",
			Handler:       _Tap_Observe_Handler,
			ServerStreams: true,
		},
	},
	Metadata: "tap.proto",
}
