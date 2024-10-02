// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.2
// 	protoc        v3.12.4
// source: http_types.proto

package http_types

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

type HttpMethod_Registered int32

const (
	HttpMethod_GET     HttpMethod_Registered = 0
	HttpMethod_POST    HttpMethod_Registered = 1
	HttpMethod_PUT     HttpMethod_Registered = 2
	HttpMethod_DELETE  HttpMethod_Registered = 3
	HttpMethod_PATCH   HttpMethod_Registered = 4
	HttpMethod_OPTIONS HttpMethod_Registered = 5
	HttpMethod_CONNECT HttpMethod_Registered = 6
	HttpMethod_HEAD    HttpMethod_Registered = 7
	HttpMethod_TRACE   HttpMethod_Registered = 8
)

// Enum value maps for HttpMethod_Registered.
var (
	HttpMethod_Registered_name = map[int32]string{
		0: "GET",
		1: "POST",
		2: "PUT",
		3: "DELETE",
		4: "PATCH",
		5: "OPTIONS",
		6: "CONNECT",
		7: "HEAD",
		8: "TRACE",
	}
	HttpMethod_Registered_value = map[string]int32{
		"GET":     0,
		"POST":    1,
		"PUT":     2,
		"DELETE":  3,
		"PATCH":   4,
		"OPTIONS": 5,
		"CONNECT": 6,
		"HEAD":    7,
		"TRACE":   8,
	}
)

func (x HttpMethod_Registered) Enum() *HttpMethod_Registered {
	p := new(HttpMethod_Registered)
	*p = x
	return p
}

func (x HttpMethod_Registered) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (HttpMethod_Registered) Descriptor() protoreflect.EnumDescriptor {
	return file_http_types_proto_enumTypes[0].Descriptor()
}

func (HttpMethod_Registered) Type() protoreflect.EnumType {
	return &file_http_types_proto_enumTypes[0]
}

func (x HttpMethod_Registered) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use HttpMethod_Registered.Descriptor instead.
func (HttpMethod_Registered) EnumDescriptor() ([]byte, []int) {
	return file_http_types_proto_rawDescGZIP(), []int{0, 0}
}

type Scheme_Registered int32

const (
	Scheme_HTTP  Scheme_Registered = 0
	Scheme_HTTPS Scheme_Registered = 1
)

// Enum value maps for Scheme_Registered.
var (
	Scheme_Registered_name = map[int32]string{
		0: "HTTP",
		1: "HTTPS",
	}
	Scheme_Registered_value = map[string]int32{
		"HTTP":  0,
		"HTTPS": 1,
	}
)

func (x Scheme_Registered) Enum() *Scheme_Registered {
	p := new(Scheme_Registered)
	*p = x
	return p
}

func (x Scheme_Registered) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (Scheme_Registered) Descriptor() protoreflect.EnumDescriptor {
	return file_http_types_proto_enumTypes[1].Descriptor()
}

func (Scheme_Registered) Type() protoreflect.EnumType {
	return &file_http_types_proto_enumTypes[1]
}

func (x Scheme_Registered) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use Scheme_Registered.Descriptor instead.
func (Scheme_Registered) EnumDescriptor() ([]byte, []int) {
	return file_http_types_proto_rawDescGZIP(), []int{1, 0}
}

type HttpMethod struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Type:
	//
	//	*HttpMethod_Registered_
	//	*HttpMethod_Unregistered
	Type isHttpMethod_Type `protobuf_oneof:"type"`
}

func (x *HttpMethod) Reset() {
	*x = HttpMethod{}
	if protoimpl.UnsafeEnabled {
		mi := &file_http_types_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *HttpMethod) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*HttpMethod) ProtoMessage() {}

func (x *HttpMethod) ProtoReflect() protoreflect.Message {
	mi := &file_http_types_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use HttpMethod.ProtoReflect.Descriptor instead.
func (*HttpMethod) Descriptor() ([]byte, []int) {
	return file_http_types_proto_rawDescGZIP(), []int{0}
}

func (m *HttpMethod) GetType() isHttpMethod_Type {
	if m != nil {
		return m.Type
	}
	return nil
}

func (x *HttpMethod) GetRegistered() HttpMethod_Registered {
	if x, ok := x.GetType().(*HttpMethod_Registered_); ok {
		return x.Registered
	}
	return HttpMethod_GET
}

func (x *HttpMethod) GetUnregistered() string {
	if x, ok := x.GetType().(*HttpMethod_Unregistered); ok {
		return x.Unregistered
	}
	return ""
}

type isHttpMethod_Type interface {
	isHttpMethod_Type()
}

type HttpMethod_Registered_ struct {
	Registered HttpMethod_Registered `protobuf:"varint,1,opt,name=registered,proto3,enum=io.linkerd.proxy.http_types.HttpMethod_Registered,oneof"`
}

type HttpMethod_Unregistered struct {
	Unregistered string `protobuf:"bytes,2,opt,name=unregistered,proto3,oneof"`
}

func (*HttpMethod_Registered_) isHttpMethod_Type() {}

func (*HttpMethod_Unregistered) isHttpMethod_Type() {}

type Scheme struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Type:
	//
	//	*Scheme_Registered_
	//	*Scheme_Unregistered
	Type isScheme_Type `protobuf_oneof:"type"`
}

func (x *Scheme) Reset() {
	*x = Scheme{}
	if protoimpl.UnsafeEnabled {
		mi := &file_http_types_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Scheme) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Scheme) ProtoMessage() {}

func (x *Scheme) ProtoReflect() protoreflect.Message {
	mi := &file_http_types_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Scheme.ProtoReflect.Descriptor instead.
func (*Scheme) Descriptor() ([]byte, []int) {
	return file_http_types_proto_rawDescGZIP(), []int{1}
}

func (m *Scheme) GetType() isScheme_Type {
	if m != nil {
		return m.Type
	}
	return nil
}

func (x *Scheme) GetRegistered() Scheme_Registered {
	if x, ok := x.GetType().(*Scheme_Registered_); ok {
		return x.Registered
	}
	return Scheme_HTTP
}

func (x *Scheme) GetUnregistered() string {
	if x, ok := x.GetType().(*Scheme_Unregistered); ok {
		return x.Unregistered
	}
	return ""
}

type isScheme_Type interface {
	isScheme_Type()
}

type Scheme_Registered_ struct {
	Registered Scheme_Registered `protobuf:"varint,1,opt,name=registered,proto3,enum=io.linkerd.proxy.http_types.Scheme_Registered,oneof"`
}

type Scheme_Unregistered struct {
	Unregistered string `protobuf:"bytes,2,opt,name=unregistered,proto3,oneof"`
}

func (*Scheme_Registered_) isScheme_Type() {}

func (*Scheme_Unregistered) isScheme_Type() {}

type Headers struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Headers []*Headers_Header `protobuf:"bytes,1,rep,name=headers,proto3" json:"headers,omitempty"`
}

func (x *Headers) Reset() {
	*x = Headers{}
	if protoimpl.UnsafeEnabled {
		mi := &file_http_types_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Headers) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Headers) ProtoMessage() {}

func (x *Headers) ProtoReflect() protoreflect.Message {
	mi := &file_http_types_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Headers.ProtoReflect.Descriptor instead.
func (*Headers) Descriptor() ([]byte, []int) {
	return file_http_types_proto_rawDescGZIP(), []int{2}
}

func (x *Headers) GetHeaders() []*Headers_Header {
	if x != nil {
		return x.Headers
	}
	return nil
}

type Headers_Header struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name  string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	Value []byte `protobuf:"bytes,2,opt,name=value,proto3" json:"value,omitempty"`
}

func (x *Headers_Header) Reset() {
	*x = Headers_Header{}
	if protoimpl.UnsafeEnabled {
		mi := &file_http_types_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Headers_Header) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Headers_Header) ProtoMessage() {}

func (x *Headers_Header) ProtoReflect() protoreflect.Message {
	mi := &file_http_types_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Headers_Header.ProtoReflect.Descriptor instead.
func (*Headers_Header) Descriptor() ([]byte, []int) {
	return file_http_types_proto_rawDescGZIP(), []int{2, 0}
}

func (x *Headers_Header) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Headers_Header) GetValue() []byte {
	if x != nil {
		return x.Value
	}
	return nil
}

var File_http_types_proto protoreflect.FileDescriptor

var file_http_types_proto_rawDesc = []byte{
	0x0a, 0x10, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x12, 0x1b, 0x69, 0x6f, 0x2e, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x2e, 0x70,
	0x72, 0x6f, 0x78, 0x79, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x22,
	0x80, 0x02, 0x0a, 0x0a, 0x48, 0x74, 0x74, 0x70, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x12, 0x54,
	0x0a, 0x0a, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x0e, 0x32, 0x32, 0x2e, 0x69, 0x6f, 0x2e, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x2e,
	0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73,
	0x2e, 0x48, 0x74, 0x74, 0x70, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x52, 0x65, 0x67, 0x69,
	0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x48, 0x00, 0x52, 0x0a, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74,
	0x65, 0x72, 0x65, 0x64, 0x12, 0x24, 0x0a, 0x0c, 0x75, 0x6e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74,
	0x65, 0x72, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0c, 0x75, 0x6e,
	0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x22, 0x6e, 0x0a, 0x0a, 0x52, 0x65,
	0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x12, 0x07, 0x0a, 0x03, 0x47, 0x45, 0x54, 0x10,
	0x00, 0x12, 0x08, 0x0a, 0x04, 0x50, 0x4f, 0x53, 0x54, 0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x50,
	0x55, 0x54, 0x10, 0x02, 0x12, 0x0a, 0x0a, 0x06, 0x44, 0x45, 0x4c, 0x45, 0x54, 0x45, 0x10, 0x03,
	0x12, 0x09, 0x0a, 0x05, 0x50, 0x41, 0x54, 0x43, 0x48, 0x10, 0x04, 0x12, 0x0b, 0x0a, 0x07, 0x4f,
	0x50, 0x54, 0x49, 0x4f, 0x4e, 0x53, 0x10, 0x05, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x4f, 0x4e, 0x4e,
	0x45, 0x43, 0x54, 0x10, 0x06, 0x12, 0x08, 0x0a, 0x04, 0x48, 0x45, 0x41, 0x44, 0x10, 0x07, 0x12,
	0x09, 0x0a, 0x05, 0x54, 0x52, 0x41, 0x43, 0x45, 0x10, 0x08, 0x42, 0x06, 0x0a, 0x04, 0x74, 0x79,
	0x70, 0x65, 0x22, 0xab, 0x01, 0x0a, 0x06, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x12, 0x50, 0x0a,
	0x0a, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x0e, 0x32, 0x2e, 0x2e, 0x69, 0x6f, 0x2e, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x2e, 0x70,
	0x72, 0x6f, 0x78, 0x79, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e,
	0x53, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65,
	0x64, 0x48, 0x00, 0x52, 0x0a, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x12,
	0x24, 0x0a, 0x0c, 0x75, 0x6e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0c, 0x75, 0x6e, 0x72, 0x65, 0x67, 0x69, 0x73,
	0x74, 0x65, 0x72, 0x65, 0x64, 0x22, 0x21, 0x0a, 0x0a, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65,
	0x72, 0x65, 0x64, 0x12, 0x08, 0x0a, 0x04, 0x48, 0x54, 0x54, 0x50, 0x10, 0x00, 0x12, 0x09, 0x0a,
	0x05, 0x48, 0x54, 0x54, 0x50, 0x53, 0x10, 0x01, 0x42, 0x06, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
	0x22, 0x84, 0x01, 0x0a, 0x07, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x12, 0x45, 0x0a, 0x07,
	0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2b, 0x2e,
	0x69, 0x6f, 0x2e, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x78, 0x79,
	0x2e, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x48, 0x65, 0x61, 0x64,
	0x65, 0x72, 0x73, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x07, 0x68, 0x65, 0x61, 0x64,
	0x65, 0x72, 0x73, 0x1a, 0x32, 0x0a, 0x06, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x12, 0x0a,
	0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d,
	0x65, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c,
	0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x35, 0x5a, 0x33, 0x67, 0x69, 0x74, 0x68, 0x75,
	0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x2f, 0x6c, 0x69,
	0x6e, 0x6b, 0x65, 0x72, 0x64, 0x32, 0x2d, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2d, 0x61, 0x70, 0x69,
	0x2f, 0x67, 0x6f, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x62, 0x06,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_http_types_proto_rawDescOnce sync.Once
	file_http_types_proto_rawDescData = file_http_types_proto_rawDesc
)

func file_http_types_proto_rawDescGZIP() []byte {
	file_http_types_proto_rawDescOnce.Do(func() {
		file_http_types_proto_rawDescData = protoimpl.X.CompressGZIP(file_http_types_proto_rawDescData)
	})
	return file_http_types_proto_rawDescData
}

var file_http_types_proto_enumTypes = make([]protoimpl.EnumInfo, 2)
var file_http_types_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_http_types_proto_goTypes = []any{
	(HttpMethod_Registered)(0), // 0: io.linkerd.proxy.http_types.HttpMethod.Registered
	(Scheme_Registered)(0),     // 1: io.linkerd.proxy.http_types.Scheme.Registered
	(*HttpMethod)(nil),         // 2: io.linkerd.proxy.http_types.HttpMethod
	(*Scheme)(nil),             // 3: io.linkerd.proxy.http_types.Scheme
	(*Headers)(nil),            // 4: io.linkerd.proxy.http_types.Headers
	(*Headers_Header)(nil),     // 5: io.linkerd.proxy.http_types.Headers.Header
}
var file_http_types_proto_depIdxs = []int32{
	0, // 0: io.linkerd.proxy.http_types.HttpMethod.registered:type_name -> io.linkerd.proxy.http_types.HttpMethod.Registered
	1, // 1: io.linkerd.proxy.http_types.Scheme.registered:type_name -> io.linkerd.proxy.http_types.Scheme.Registered
	5, // 2: io.linkerd.proxy.http_types.Headers.headers:type_name -> io.linkerd.proxy.http_types.Headers.Header
	3, // [3:3] is the sub-list for method output_type
	3, // [3:3] is the sub-list for method input_type
	3, // [3:3] is the sub-list for extension type_name
	3, // [3:3] is the sub-list for extension extendee
	0, // [0:3] is the sub-list for field type_name
}

func init() { file_http_types_proto_init() }
func file_http_types_proto_init() {
	if File_http_types_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_http_types_proto_msgTypes[0].Exporter = func(v any, i int) any {
			switch v := v.(*HttpMethod); i {
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
		file_http_types_proto_msgTypes[1].Exporter = func(v any, i int) any {
			switch v := v.(*Scheme); i {
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
		file_http_types_proto_msgTypes[2].Exporter = func(v any, i int) any {
			switch v := v.(*Headers); i {
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
		file_http_types_proto_msgTypes[3].Exporter = func(v any, i int) any {
			switch v := v.(*Headers_Header); i {
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
	file_http_types_proto_msgTypes[0].OneofWrappers = []any{
		(*HttpMethod_Registered_)(nil),
		(*HttpMethod_Unregistered)(nil),
	}
	file_http_types_proto_msgTypes[1].OneofWrappers = []any{
		(*Scheme_Registered_)(nil),
		(*Scheme_Unregistered)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_http_types_proto_rawDesc,
			NumEnums:      2,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_http_types_proto_goTypes,
		DependencyIndexes: file_http_types_proto_depIdxs,
		EnumInfos:         file_http_types_proto_enumTypes,
		MessageInfos:      file_http_types_proto_msgTypes,
	}.Build()
	File_http_types_proto = out.File
	file_http_types_proto_rawDesc = nil
	file_http_types_proto_goTypes = nil
	file_http_types_proto_depIdxs = nil
}
