// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.35.2
// 	protoc        v5.29.0
// source: meta.proto

package meta

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

// General metadata about a configuration object. Typically references either an
// implicit default configuration or a Kubernetes resource.
type Metadata struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Kind:
	//
	//	*Metadata_Default
	//	*Metadata_Resource
	Kind isMetadata_Kind `protobuf_oneof:"kind"`
}

func (x *Metadata) Reset() {
	*x = Metadata{}
	mi := &file_meta_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *Metadata) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Metadata) ProtoMessage() {}

func (x *Metadata) ProtoReflect() protoreflect.Message {
	mi := &file_meta_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Metadata.ProtoReflect.Descriptor instead.
func (*Metadata) Descriptor() ([]byte, []int) {
	return file_meta_proto_rawDescGZIP(), []int{0}
}

func (m *Metadata) GetKind() isMetadata_Kind {
	if m != nil {
		return m.Kind
	}
	return nil
}

func (x *Metadata) GetDefault() string {
	if x, ok := x.GetKind().(*Metadata_Default); ok {
		return x.Default
	}
	return ""
}

func (x *Metadata) GetResource() *Resource {
	if x, ok := x.GetKind().(*Metadata_Resource); ok {
		return x.Resource
	}
	return nil
}

type isMetadata_Kind interface {
	isMetadata_Kind()
}

type Metadata_Default struct {
	// A name describing a default/implicit configuration.
	//
	// For example, a policy default name like `all-authenticated` describes an
	// implicit controller-implementedc policy that does not exist as a cluster
	// resource.
	Default string `protobuf:"bytes,1,opt,name=default,proto3,oneof"`
}

type Metadata_Resource struct {
	Resource *Resource `protobuf:"bytes,2,opt,name=resource,proto3,oneof"`
}

func (*Metadata_Default) isMetadata_Kind() {}

func (*Metadata_Resource) isMetadata_Kind() {}

// References a (e.g., Kubernetes) resource.
type Resource struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Group     string `protobuf:"bytes,1,opt,name=group,proto3" json:"group,omitempty"`
	Kind      string `protobuf:"bytes,2,opt,name=kind,proto3" json:"kind,omitempty"`
	Name      string `protobuf:"bytes,3,opt,name=name,proto3" json:"name,omitempty"`
	Namespace string `protobuf:"bytes,4,opt,name=namespace,proto3" json:"namespace,omitempty"`
	Section   string `protobuf:"bytes,5,opt,name=section,proto3" json:"section,omitempty"`
	Port      uint32 `protobuf:"varint,6,opt,name=port,proto3" json:"port,omitempty"`
}

func (x *Resource) Reset() {
	*x = Resource{}
	mi := &file_meta_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *Resource) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Resource) ProtoMessage() {}

func (x *Resource) ProtoReflect() protoreflect.Message {
	mi := &file_meta_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Resource.ProtoReflect.Descriptor instead.
func (*Resource) Descriptor() ([]byte, []int) {
	return file_meta_proto_rawDescGZIP(), []int{1}
}

func (x *Resource) GetGroup() string {
	if x != nil {
		return x.Group
	}
	return ""
}

func (x *Resource) GetKind() string {
	if x != nil {
		return x.Kind
	}
	return ""
}

func (x *Resource) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Resource) GetNamespace() string {
	if x != nil {
		return x.Namespace
	}
	return ""
}

func (x *Resource) GetSection() string {
	if x != nil {
		return x.Section
	}
	return ""
}

func (x *Resource) GetPort() uint32 {
	if x != nil {
		return x.Port
	}
	return 0
}

var File_meta_proto protoreflect.FileDescriptor

var file_meta_proto_rawDesc = []byte{
	0x0a, 0x0a, 0x6d, 0x65, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x15, 0x69, 0x6f,
	0x2e, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x6d,
	0x65, 0x74, 0x61, 0x22, 0x6d, 0x0a, 0x08, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12,
	0x1a, 0x0a, 0x07, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x48, 0x00, 0x52, 0x07, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x12, 0x3d, 0x0a, 0x08, 0x72,
	0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e,
	0x69, 0x6f, 0x2e, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x78, 0x79,
	0x2e, 0x6d, 0x65, 0x74, 0x61, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x48, 0x00,
	0x52, 0x08, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x42, 0x06, 0x0a, 0x04, 0x6b, 0x69,
	0x6e, 0x64, 0x22, 0x94, 0x01, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12,
	0x14, 0x0a, 0x05, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05,
	0x67, 0x72, 0x6f, 0x75, 0x70, 0x12, 0x12, 0x0a, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
	0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1c, 0x0a,
	0x09, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x09, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73,
	0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x65,
	0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x06, 0x20,
	0x01, 0x28, 0x0d, 0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x42, 0x2f, 0x5a, 0x2d, 0x67, 0x69, 0x74,
	0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x2f,
	0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x64, 0x32, 0x2d, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2d, 0x61,
	0x70, 0x69, 0x2f, 0x67, 0x6f, 0x2f, 0x6d, 0x65, 0x74, 0x61, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x33,
}

var (
	file_meta_proto_rawDescOnce sync.Once
	file_meta_proto_rawDescData = file_meta_proto_rawDesc
)

func file_meta_proto_rawDescGZIP() []byte {
	file_meta_proto_rawDescOnce.Do(func() {
		file_meta_proto_rawDescData = protoimpl.X.CompressGZIP(file_meta_proto_rawDescData)
	})
	return file_meta_proto_rawDescData
}

var file_meta_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_meta_proto_goTypes = []any{
	(*Metadata)(nil), // 0: io.linkerd.proxy.meta.Metadata
	(*Resource)(nil), // 1: io.linkerd.proxy.meta.Resource
}
var file_meta_proto_depIdxs = []int32{
	1, // 0: io.linkerd.proxy.meta.Metadata.resource:type_name -> io.linkerd.proxy.meta.Resource
	1, // [1:1] is the sub-list for method output_type
	1, // [1:1] is the sub-list for method input_type
	1, // [1:1] is the sub-list for extension type_name
	1, // [1:1] is the sub-list for extension extendee
	0, // [0:1] is the sub-list for field type_name
}

func init() { file_meta_proto_init() }
func file_meta_proto_init() {
	if File_meta_proto != nil {
		return
	}
	file_meta_proto_msgTypes[0].OneofWrappers = []any{
		(*Metadata_Default)(nil),
		(*Metadata_Resource)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_meta_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_meta_proto_goTypes,
		DependencyIndexes: file_meta_proto_depIdxs,
		MessageInfos:      file_meta_proto_msgTypes,
	}.Build()
	File_meta_proto = out.File
	file_meta_proto_rawDesc = nil
	file_meta_proto_goTypes = nil
	file_meta_proto_depIdxs = nil
}
