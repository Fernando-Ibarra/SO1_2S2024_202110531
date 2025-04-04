// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.2
// 	protoc        v5.28.2
// source: athlete.proto

package proto

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

type AthleteRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Student    string `protobuf:"bytes,1,opt,name=student,proto3" json:"student,omitempty"`
	Age        int64  `protobuf:"varint,2,opt,name=age,proto3" json:"age,omitempty"`
	Faculty    string `protobuf:"bytes,3,opt,name=faculty,proto3" json:"faculty,omitempty"`
	Discipline int64  `protobuf:"varint,4,opt,name=discipline,proto3" json:"discipline,omitempty"`
}

func (x *AthleteRequest) Reset() {
	*x = AthleteRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_athlete_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AthleteRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AthleteRequest) ProtoMessage() {}

func (x *AthleteRequest) ProtoReflect() protoreflect.Message {
	mi := &file_athlete_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AthleteRequest.ProtoReflect.Descriptor instead.
func (*AthleteRequest) Descriptor() ([]byte, []int) {
	return file_athlete_proto_rawDescGZIP(), []int{0}
}

func (x *AthleteRequest) GetStudent() string {
	if x != nil {
		return x.Student
	}
	return ""
}

func (x *AthleteRequest) GetAge() int64 {
	if x != nil {
		return x.Age
	}
	return 0
}

func (x *AthleteRequest) GetFaculty() string {
	if x != nil {
		return x.Faculty
	}
	return ""
}

func (x *AthleteRequest) GetDiscipline() int64 {
	if x != nil {
		return x.Discipline
	}
	return 0
}

type AthleteResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Student string `protobuf:"bytes,1,opt,name=student,proto3" json:"student,omitempty"`
}

func (x *AthleteResponse) Reset() {
	*x = AthleteResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_athlete_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AthleteResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AthleteResponse) ProtoMessage() {}

func (x *AthleteResponse) ProtoReflect() protoreflect.Message {
	mi := &file_athlete_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AthleteResponse.ProtoReflect.Descriptor instead.
func (*AthleteResponse) Descriptor() ([]byte, []int) {
	return file_athlete_proto_rawDescGZIP(), []int{1}
}

func (x *AthleteResponse) GetStudent() string {
	if x != nil {
		return x.Student
	}
	return ""
}

var File_athlete_proto protoreflect.FileDescriptor

var file_athlete_proto_rawDesc = []byte{
	0x0a, 0x0d, 0x61, 0x74, 0x68, 0x6c, 0x65, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
	0x04, 0x6d, 0x61, 0x69, 0x6e, 0x22, 0x76, 0x0a, 0x0e, 0x41, 0x74, 0x68, 0x6c, 0x65, 0x74, 0x65,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x75, 0x64, 0x65,
	0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x74, 0x75, 0x64, 0x65, 0x6e,
	0x74, 0x12, 0x10, 0x0a, 0x03, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x03,
	0x61, 0x67, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x66, 0x61, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x18, 0x03,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x66, 0x61, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x12, 0x1e, 0x0a,
	0x0a, 0x64, 0x69, 0x73, 0x63, 0x69, 0x70, 0x6c, 0x69, 0x6e, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
	0x03, 0x52, 0x0a, 0x64, 0x69, 0x73, 0x63, 0x69, 0x70, 0x6c, 0x69, 0x6e, 0x65, 0x22, 0x2b, 0x0a,
	0x0f, 0x41, 0x74, 0x68, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
	0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x75, 0x64, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x07, 0x73, 0x74, 0x75, 0x64, 0x65, 0x6e, 0x74, 0x32, 0x4d, 0x0a, 0x0b, 0x41, 0x74,
	0x68, 0x6c, 0x65, 0x74, 0x65, 0x75, 0x69, 0x64, 0x65, 0x12, 0x3e, 0x0a, 0x0d, 0x43, 0x72, 0x65,
	0x61, 0x74, 0x65, 0x41, 0x74, 0x68, 0x6c, 0x65, 0x74, 0x65, 0x12, 0x14, 0x2e, 0x6d, 0x61, 0x69,
	0x6e, 0x2e, 0x41, 0x74, 0x68, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
	0x1a, 0x15, 0x2e, 0x6d, 0x61, 0x69, 0x6e, 0x2e, 0x41, 0x74, 0x68, 0x6c, 0x65, 0x74, 0x65, 0x52,
	0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x42, 0x09, 0x5a, 0x07, 0x2e, 0x2f, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_athlete_proto_rawDescOnce sync.Once
	file_athlete_proto_rawDescData = file_athlete_proto_rawDesc
)

func file_athlete_proto_rawDescGZIP() []byte {
	file_athlete_proto_rawDescOnce.Do(func() {
		file_athlete_proto_rawDescData = protoimpl.X.CompressGZIP(file_athlete_proto_rawDescData)
	})
	return file_athlete_proto_rawDescData
}

var file_athlete_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_athlete_proto_goTypes = []any{
	(*AthleteRequest)(nil),  // 0: main.AthleteRequest
	(*AthleteResponse)(nil), // 1: main.AthleteResponse
}
var file_athlete_proto_depIdxs = []int32{
	0, // 0: main.Athleteuide.CreateAthlete:input_type -> main.AthleteRequest
	1, // 1: main.Athleteuide.CreateAthlete:output_type -> main.AthleteResponse
	1, // [1:2] is the sub-list for method output_type
	0, // [0:1] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_athlete_proto_init() }
func file_athlete_proto_init() {
	if File_athlete_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_athlete_proto_msgTypes[0].Exporter = func(v any, i int) any {
			switch v := v.(*AthleteRequest); i {
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
		file_athlete_proto_msgTypes[1].Exporter = func(v any, i int) any {
			switch v := v.(*AthleteResponse); i {
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
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_athlete_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_athlete_proto_goTypes,
		DependencyIndexes: file_athlete_proto_depIdxs,
		MessageInfos:      file_athlete_proto_msgTypes,
	}.Build()
	File_athlete_proto = out.File
	file_athlete_proto_rawDesc = nil
	file_athlete_proto_goTypes = nil
	file_athlete_proto_depIdxs = nil
}
