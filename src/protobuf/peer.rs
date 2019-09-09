// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `peer.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
pub struct NetMessage {
    // message oneof groups
    pub Type: ::std::option::Option<NetMessage_oneof_Type>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a NetMessage {
    fn default() -> &'a NetMessage {
        <NetMessage as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum NetMessage_oneof_Type {
    data(::std::vec::Vec<u8>),
    config_change(super::eraftpb::ConfChange),
    raft_message(super::eraftpb::Message),
}

impl NetMessage {
    pub fn new() -> NetMessage {
        ::std::default::Default::default()
    }

    // bytes data = 1;


    pub fn get_data(&self) -> &[u8] {
        match self.Type {
            ::std::option::Option::Some(NetMessage_oneof_Type::data(ref v)) => v,
            _ => &[],
        }
    }
    pub fn clear_data(&mut self) {
        self.Type = ::std::option::Option::None;
    }

    pub fn has_data(&self) -> bool {
        match self.Type {
            ::std::option::Option::Some(NetMessage_oneof_Type::data(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.Type = ::std::option::Option::Some(NetMessage_oneof_Type::data(v))
    }

    // Mutable pointer to the field.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(NetMessage_oneof_Type::data(_)) = self.Type {
        } else {
            self.Type = ::std::option::Option::Some(NetMessage_oneof_Type::data(::std::vec::Vec::new()));
        }
        match self.Type {
            ::std::option::Option::Some(NetMessage_oneof_Type::data(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_data() {
            match self.Type.take() {
                ::std::option::Option::Some(NetMessage_oneof_Type::data(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    // .eraftpb.ConfChange config_change = 2;


    pub fn get_config_change(&self) -> &super::eraftpb::ConfChange {
        match self.Type {
            ::std::option::Option::Some(NetMessage_oneof_Type::config_change(ref v)) => v,
            _ => super::eraftpb::ConfChange::default_instance(),
        }
    }
    pub fn clear_config_change(&mut self) {
        self.Type = ::std::option::Option::None;
    }

    pub fn has_config_change(&self) -> bool {
        match self.Type {
            ::std::option::Option::Some(NetMessage_oneof_Type::config_change(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_config_change(&mut self, v: super::eraftpb::ConfChange) {
        self.Type = ::std::option::Option::Some(NetMessage_oneof_Type::config_change(v))
    }

    // Mutable pointer to the field.
    pub fn mut_config_change(&mut self) -> &mut super::eraftpb::ConfChange {
        if let ::std::option::Option::Some(NetMessage_oneof_Type::config_change(_)) = self.Type {
        } else {
            self.Type = ::std::option::Option::Some(NetMessage_oneof_Type::config_change(super::eraftpb::ConfChange::new()));
        }
        match self.Type {
            ::std::option::Option::Some(NetMessage_oneof_Type::config_change(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_config_change(&mut self) -> super::eraftpb::ConfChange {
        if self.has_config_change() {
            match self.Type.take() {
                ::std::option::Option::Some(NetMessage_oneof_Type::config_change(v)) => v,
                _ => panic!(),
            }
        } else {
            super::eraftpb::ConfChange::new()
        }
    }

    // .eraftpb.Message raft_message = 3;


    pub fn get_raft_message(&self) -> &super::eraftpb::Message {
        match self.Type {
            ::std::option::Option::Some(NetMessage_oneof_Type::raft_message(ref v)) => v,
            _ => super::eraftpb::Message::default_instance(),
        }
    }
    pub fn clear_raft_message(&mut self) {
        self.Type = ::std::option::Option::None;
    }

    pub fn has_raft_message(&self) -> bool {
        match self.Type {
            ::std::option::Option::Some(NetMessage_oneof_Type::raft_message(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_raft_message(&mut self, v: super::eraftpb::Message) {
        self.Type = ::std::option::Option::Some(NetMessage_oneof_Type::raft_message(v))
    }

    // Mutable pointer to the field.
    pub fn mut_raft_message(&mut self) -> &mut super::eraftpb::Message {
        if let ::std::option::Option::Some(NetMessage_oneof_Type::raft_message(_)) = self.Type {
        } else {
            self.Type = ::std::option::Option::Some(NetMessage_oneof_Type::raft_message(super::eraftpb::Message::new()));
        }
        match self.Type {
            ::std::option::Option::Some(NetMessage_oneof_Type::raft_message(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_raft_message(&mut self) -> super::eraftpb::Message {
        if self.has_raft_message() {
            match self.Type.take() {
                ::std::option::Option::Some(NetMessage_oneof_Type::raft_message(v)) => v,
                _ => panic!(),
            }
        } else {
            super::eraftpb::Message::new()
        }
    }
}

impl ::protobuf::Message for NetMessage {
    fn is_initialized(&self) -> bool {
        if let Some(NetMessage_oneof_Type::config_change(ref v)) = self.Type {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(NetMessage_oneof_Type::raft_message(ref v)) = self.Type {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Type = ::std::option::Option::Some(NetMessage_oneof_Type::data(is.read_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Type = ::std::option::Option::Some(NetMessage_oneof_Type::config_change(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Type = ::std::option::Option::Some(NetMessage_oneof_Type::raft_message(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.Type {
            match v {
                &NetMessage_oneof_Type::data(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &NetMessage_oneof_Type::config_change(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &NetMessage_oneof_Type::raft_message(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.Type {
            match v {
                &NetMessage_oneof_Type::data(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &NetMessage_oneof_Type::config_change(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &NetMessage_oneof_Type::raft_message(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> NetMessage {
        NetMessage::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "data",
                    NetMessage::has_data,
                    NetMessage::get_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::eraftpb::ConfChange>(
                    "config_change",
                    NetMessage::has_config_change,
                    NetMessage::get_config_change,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::eraftpb::Message>(
                    "raft_message",
                    NetMessage::has_raft_message,
                    NetMessage::get_raft_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NetMessage>(
                    "NetMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static NetMessage {
        static mut instance: ::protobuf::lazy::Lazy<NetMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetMessage,
        };
        unsafe {
            instance.get(NetMessage::new)
        }
    }
}

impl ::protobuf::Clear for NetMessage {
    fn clear(&mut self) {
        self.Type = ::std::option::Option::None;
        self.Type = ::std::option::Option::None;
        self.Type = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\npeer.proto\x12\x05proto\x1a\x0ccommon.proto\x1a\reraftpb.proto\"\x9d\
    \x01\n\nNetMessage\x12\x14\n\x04data\x18\x01\x20\x01(\x0cH\0R\x04data\
    \x12:\n\rconfig_change\x18\x02\x20\x01(\x0b2\x13.eraftpb.ConfChangeH\0R\
    \x0cconfigChange\x125\n\x0craft_message\x18\x03\x20\x01(\x0b2\x10.eraftp\
    b.MessageH\0R\x0braftMessageB\x06\n\x04Type2O\n\x08Endorser\x12C\n\x0fPr\
    ocessProposal\x12\x15.proto.SignedProposal\x1a\x17.proto.ProposalRespons\
    e\"\027\n\x07Network\x12,\n\tBroadcast\x12\x0f.proto.Envelope\x1a\x0c.pr\
    oto.Empty\"\0b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
