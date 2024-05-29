// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.2
// 	protoc        v3.20.3
// source: net.proto

package net

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type IPAddress struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Ip:
	//
	//	*IPAddress_Ipv4
	//	*IPAddress_Ipv6
	Ip isIPAddress_Ip `protobuf_oneof:"ip"`
}

func (x *IPAddress) Reset() {
	*x = IPAddress{}
	if protoimpl.UnsafeEnabled {
		mi := &file_net_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *IPAddress) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IPAddress) ProtoMessage() {}

func (x *IPAddress) ProtoReflect() protoreflect.Message {
	mi := &file_net_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IPAddress.ProtoReflect.Descriptor instead.
func (*IPAddress) Descriptor() ([]byte, []int) {
	return file_net_proto_rawDescGZIP(), []int{0}
}

func (m *IPAddress) GetIp() isIPAddress_Ip {
	if m != nil {
		return m.Ip
	}
	return nil
}

func (x *IPAddress) GetIpv4() uint32 {
	if x, ok := x.GetIp().(*IPAddress_Ipv4); ok {
		return x.Ipv4
	}
	return 0
}

func (x *IPAddress) GetIpv6() *IPv6 {
	if x, ok := x.GetIp().(*IPAddress_Ipv6); ok {
		return x.Ipv6
	}
	return nil
}

type isIPAddress_Ip interface {
	isIPAddress_Ip()
}

type IPAddress_Ipv4 struct {
	Ipv4 uint32 `protobuf:"fixed32,1,opt,name=ipv4,proto3,oneof"`
}

type IPAddress_Ipv6 struct {
	Ipv6 *IPv6 `protobuf:"bytes,2,opt,name=ipv6,proto3,oneof"`
}

func (*IPAddress_Ipv4) isIPAddress_Ip() {}

func (*IPAddress_Ipv6) isIPAddress_Ip() {}

type IPNetwork struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Ip        *IPAddress `protobuf:"bytes,1,opt,name=ip,proto3" json:"ip,omitempty"`
	PrefixLen uint32     `protobuf:"varint,2,opt,name=prefix_len,json=prefixLen,proto3" json:"prefix_len,omitempty"`
}

func (x *IPNetwork) Reset() {
	*x = IPNetwork{}
	if protoimpl.UnsafeEnabled {
		mi := &file_net_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *IPNetwork) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IPNetwork) ProtoMessage() {}

func (x *IPNetwork) ProtoReflect() protoreflect.Message {
	mi := &file_net_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IPNetwork.ProtoReflect.Descriptor instead.
func (*IPNetwork) Descriptor() ([]byte, []int) {
	return file_net_proto_rawDescGZIP(), []int{1}
}

func (x *IPNetwork) GetIp() *IPAddress {
	if x != nil {
		return x.Ip
	}
	return nil
}

func (x *IPNetwork) GetPrefixLen() uint32 {
	if x != nil {
		return x.PrefixLen
	}
	return 0
}

type IPv6 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	First uint64 `protobuf:"fixed64,1,opt,name=first,proto3" json:"first,omitempty"` // hextets 1-4
	Last  uint64 `protobuf:"fixed64,2,opt,name=last,proto3" json:"last,omitempty"`   // hextets 5-8
}

func (x *IPv6) Reset() {
	*x = IPv6{}
	if protoimpl.UnsafeEnabled {
		mi := &file_net_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *IPv6) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IPv6) ProtoMessage() {}

func (x *IPv6) ProtoReflect() protoreflect.Message {
	mi := &file_net_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IPv6.ProtoReflect.Descriptor instead.
func (*IPv6) Descriptor() ([]byte, []int) {
	return file_net_proto_rawDescGZIP(), []int{2}
}

func (x *IPv6) GetFirst() uint64 {
	if x != nil {
		return x.First
	}
	return 0
}

func (x *IPv6) GetLast() uint64 {
	if x != nil {
		return x.Last
	}
	return 0
}

type TcpAddress struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Ip   *IPAddress `protobuf:"bytes,1,opt,name=ip,proto3" json:"ip,omitempty"`
	Port uint32     `protobuf:"varint,2,opt,name=port,proto3" json:"port,omitempty"`
}

func (x *TcpAddress) Reset() {
	*x = TcpAddress{}
	if protoimpl.UnsafeEnabled {
		mi := &file_net_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TcpAddress) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TcpAddress) ProtoMessage() {}

func (x *TcpAddress) ProtoReflect() protoreflect.Message {
	mi := &file_net_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TcpAddress.ProtoReflect.Descriptor instead.
func (*TcpAddress) Descriptor() ([]byte, []int) {
	return file_net_proto_rawDescGZIP(), []int{3}
}

func (x *TcpAddress) GetIp() *IPAddress {
	if x != nil {
		return x.Ip
	}
	return nil
}

func (x *TcpAddress) GetPort() uint32 {
	if x != nil {
		return x.Port
	}
	return 0
}

var File_net_proto protoreflect.FileDescriptor

var file_net_proto_rawDesc = []byte{
	0x0a, 0x09, 0x6e, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x14, 0x69, 0x6f, 0x2e,
	0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x6e, 0x65,
	0x74, 0x22, 0x59, 0x0a, 0x09, 0x49, 0x50, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x14,
	0x0a, 0x04, 0x69, 0x70, 0x76, 0x34, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x48, 0x00, 0x52, 0x04,
	0x69, 0x70, 0x76, 0x34, 0x12, 0x30, 0x0a, 0x04, 0x69, 0x70, 0x76, 0x36, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x69, 0x6f, 0x2e, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x2e,
	0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x6e, 0x65, 0x74, 0x2e, 0x49, 0x50, 0x76, 0x36, 0x48, 0x00,
	0x52, 0x04, 0x69, 0x70, 0x76, 0x36, 0x42, 0x04, 0x0a, 0x02, 0x69, 0x70, 0x22, 0x5b, 0x0a, 0x09,
	0x49, 0x50, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x12, 0x2f, 0x0a, 0x02, 0x69, 0x70, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x69, 0x6f, 0x2e, 0x6c, 0x69, 0x6e, 0x6b, 0x65,
	0x72, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x6e, 0x65, 0x74, 0x2e, 0x49, 0x50, 0x41,
	0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x02, 0x69, 0x70, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x72,
	0x65, 0x66, 0x69, 0x78, 0x5f, 0x6c, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09,
	0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x4c, 0x65, 0x6e, 0x22, 0x30, 0x0a, 0x04, 0x49, 0x50, 0x76,
	0x36, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x69, 0x72, 0x73, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06,
	0x52, 0x05, 0x66, 0x69, 0x72, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6c, 0x61, 0x73, 0x74, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x04, 0x6c, 0x61, 0x73, 0x74, 0x22, 0x51, 0x0a, 0x0a, 0x54,
	0x63, 0x70, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x2f, 0x0a, 0x02, 0x69, 0x70, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x69, 0x6f, 0x2e, 0x6c, 0x69, 0x6e, 0x6b, 0x65,
	0x72, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x6e, 0x65, 0x74, 0x2e, 0x49, 0x50, 0x41,
	0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x02, 0x69, 0x70, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x6f,
	0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x42, 0x2e,
	0x5a, 0x2c, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6c, 0x69, 0x6e,
	0x6b, 0x65, 0x72, 0x64, 0x2f, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x32, 0x2d, 0x70, 0x72,
	0x6f, 0x78, 0x79, 0x2d, 0x61, 0x70, 0x69, 0x2f, 0x67, 0x6f, 0x2f, 0x6e, 0x65, 0x74, 0x62, 0x06,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_net_proto_rawDescOnce sync.Once
	file_net_proto_rawDescData = file_net_proto_rawDesc
)

func file_net_proto_rawDescGZIP() []byte {
	file_net_proto_rawDescOnce.Do(func() {
		file_net_proto_rawDescData = protoimpl.X.CompressGZIP(file_net_proto_rawDescData)
	})
	return file_net_proto_rawDescData
}

var file_net_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_net_proto_goTypes = []any{
	(*IPAddress)(nil),  // 0: io.linkerd.proxy.net.IPAddress
	(*IPNetwork)(nil),  // 1: io.linkerd.proxy.net.IPNetwork
	(*IPv6)(nil),       // 2: io.linkerd.proxy.net.IPv6
	(*TcpAddress)(nil), // 3: io.linkerd.proxy.net.TcpAddress
}
var file_net_proto_depIdxs = []int32{
	2, // 0: io.linkerd.proxy.net.IPAddress.ipv6:type_name -> io.linkerd.proxy.net.IPv6
	0, // 1: io.linkerd.proxy.net.IPNetwork.ip:type_name -> io.linkerd.proxy.net.IPAddress
	0, // 2: io.linkerd.proxy.net.TcpAddress.ip:type_name -> io.linkerd.proxy.net.IPAddress
	3, // [3:3] is the sub-list for method output_type
	3, // [3:3] is the sub-list for method input_type
	3, // [3:3] is the sub-list for extension type_name
	3, // [3:3] is the sub-list for extension extendee
	0, // [0:3] is the sub-list for field type_name
}

func init() { file_net_proto_init() }
func file_net_proto_init() {
	if File_net_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_net_proto_msgTypes[0].Exporter = func(v any, i int) any {
			switch v := v.(*IPAddress); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_net_proto_msgTypes[1].Exporter = func(v any, i int) any {
			switch v := v.(*IPNetwork); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_net_proto_msgTypes[2].Exporter = func(v any, i int) any {
			switch v := v.(*IPv6); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_net_proto_msgTypes[3].Exporter = func(v any, i int) any {
			switch v := v.(*TcpAddress); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	file_net_proto_msgTypes[0].OneofWrappers = []any{
		(*IPAddress_Ipv4)(nil),
		(*IPAddress_Ipv6)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_net_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_net_proto_goTypes,
		DependencyIndexes: file_net_proto_depIdxs,
		MessageInfos:      file_net_proto_msgTypes,
	}.Build()
	File_net_proto = out.File
	file_net_proto_rawDesc = nil
	file_net_proto_goTypes = nil
	file_net_proto_depIdxs = nil
}
