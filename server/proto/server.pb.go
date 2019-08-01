// Code generated by protoc-gen-go. DO NOT EDIT.
// source: proto/server.proto

package ast

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

type GetFileRequest struct {
	Path                 string   `protobuf:"bytes,1,opt,name=path,proto3" json:"path,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *GetFileRequest) Reset()         { *m = GetFileRequest{} }
func (m *GetFileRequest) String() string { return proto.CompactTextString(m) }
func (*GetFileRequest) ProtoMessage()    {}
func (*GetFileRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_b6e0ddbe8bedca8f, []int{0}
}

func (m *GetFileRequest) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_GetFileRequest.Unmarshal(m, b)
}
func (m *GetFileRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_GetFileRequest.Marshal(b, m, deterministic)
}
func (m *GetFileRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_GetFileRequest.Merge(m, src)
}
func (m *GetFileRequest) XXX_Size() int {
	return xxx_messageInfo_GetFileRequest.Size(m)
}
func (m *GetFileRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_GetFileRequest.DiscardUnknown(m)
}

var xxx_messageInfo_GetFileRequest proto.InternalMessageInfo

func (m *GetFileRequest) GetPath() string {
	if m != nil {
		return m.Path
	}
	return ""
}

type GetFileResponse struct {
	JsonContent          string   `protobuf:"bytes,1,opt,name=json_content,json=jsonContent,proto3" json:"json_content,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *GetFileResponse) Reset()         { *m = GetFileResponse{} }
func (m *GetFileResponse) String() string { return proto.CompactTextString(m) }
func (*GetFileResponse) ProtoMessage()    {}
func (*GetFileResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_b6e0ddbe8bedca8f, []int{1}
}

func (m *GetFileResponse) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_GetFileResponse.Unmarshal(m, b)
}
func (m *GetFileResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_GetFileResponse.Marshal(b, m, deterministic)
}
func (m *GetFileResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_GetFileResponse.Merge(m, src)
}
func (m *GetFileResponse) XXX_Size() int {
	return xxx_messageInfo_GetFileResponse.Size(m)
}
func (m *GetFileResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_GetFileResponse.DiscardUnknown(m)
}

var xxx_messageInfo_GetFileResponse proto.InternalMessageInfo

func (m *GetFileResponse) GetJsonContent() string {
	if m != nil {
		return m.JsonContent
	}
	return ""
}

type UpdateFileRequest struct {
	Path                 string   `protobuf:"bytes,1,opt,name=path,proto3" json:"path,omitempty"`
	JsonContent          string   `protobuf:"bytes,2,opt,name=json_content,json=jsonContent,proto3" json:"json_content,omitempty"`
	ElmContent           string   `protobuf:"bytes,3,opt,name=elm_content,json=elmContent,proto3" json:"elm_content,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *UpdateFileRequest) Reset()         { *m = UpdateFileRequest{} }
func (m *UpdateFileRequest) String() string { return proto.CompactTextString(m) }
func (*UpdateFileRequest) ProtoMessage()    {}
func (*UpdateFileRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_b6e0ddbe8bedca8f, []int{2}
}

func (m *UpdateFileRequest) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_UpdateFileRequest.Unmarshal(m, b)
}
func (m *UpdateFileRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_UpdateFileRequest.Marshal(b, m, deterministic)
}
func (m *UpdateFileRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_UpdateFileRequest.Merge(m, src)
}
func (m *UpdateFileRequest) XXX_Size() int {
	return xxx_messageInfo_UpdateFileRequest.Size(m)
}
func (m *UpdateFileRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_UpdateFileRequest.DiscardUnknown(m)
}

var xxx_messageInfo_UpdateFileRequest proto.InternalMessageInfo

func (m *UpdateFileRequest) GetPath() string {
	if m != nil {
		return m.Path
	}
	return ""
}

func (m *UpdateFileRequest) GetJsonContent() string {
	if m != nil {
		return m.JsonContent
	}
	return ""
}

func (m *UpdateFileRequest) GetElmContent() string {
	if m != nil {
		return m.ElmContent
	}
	return ""
}

func init() {
	proto.RegisterType((*GetFileRequest)(nil), "ast.GetFileRequest")
	proto.RegisterType((*GetFileResponse)(nil), "ast.GetFileResponse")
	proto.RegisterType((*UpdateFileRequest)(nil), "ast.UpdateFileRequest")
}

func init() { proto.RegisterFile("proto/server.proto", fileDescriptor_b6e0ddbe8bedca8f) }

var fileDescriptor_b6e0ddbe8bedca8f = []byte{
	// 157 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0x12, 0x2a, 0x28, 0xca, 0x2f,
	0xc9, 0xd7, 0x2f, 0x4e, 0x2d, 0x2a, 0x4b, 0x2d, 0xd2, 0x03, 0x73, 0x84, 0x98, 0x13, 0x8b, 0x4b,
	0x94, 0x54, 0xb8, 0xf8, 0xdc, 0x53, 0x4b, 0xdc, 0x32, 0x73, 0x52, 0x83, 0x52, 0x0b, 0x4b, 0x53,
	0x8b, 0x4b, 0x84, 0x84, 0xb8, 0x58, 0x0a, 0x12, 0x4b, 0x32, 0x24, 0x18, 0x15, 0x18, 0x35, 0x38,
	0x83, 0xc0, 0x6c, 0x25, 0x13, 0x2e, 0x7e, 0xb8, 0xaa, 0xe2, 0x82, 0xfc, 0xbc, 0xe2, 0x54, 0x21,
	0x45, 0x2e, 0x9e, 0xac, 0xe2, 0xfc, 0xbc, 0xf8, 0xe4, 0xfc, 0xbc, 0x92, 0xd4, 0xbc, 0x12, 0xa8,
	0x72, 0x6e, 0x90, 0x98, 0x33, 0x44, 0x48, 0x29, 0x9b, 0x4b, 0x30, 0xb4, 0x20, 0x25, 0xb1, 0x24,
	0x95, 0x80, 0xf1, 0x18, 0x66, 0x31, 0x61, 0x98, 0x25, 0x24, 0xcf, 0xc5, 0x9d, 0x9a, 0x93, 0x0b,
	0x57, 0xc1, 0x0c, 0x56, 0xc1, 0x95, 0x9a, 0x93, 0x0b, 0x55, 0x90, 0xc4, 0x06, 0xf6, 0x94, 0x31,
	0x20, 0x00, 0x00, 0xff, 0xff, 0x4b, 0x9a, 0x42, 0x43, 0xea, 0x00, 0x00, 0x00,
}