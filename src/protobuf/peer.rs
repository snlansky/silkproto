// This file is generated by rust-protobuf 2.8.0. Do not edit
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
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    pub field_type: Message_Type,
    pub Chain: ::std::string::String,
    pub from: ::std::string::String,
    pub Target: ::protobuf::RepeatedField<::std::string::String>,
    pub Payload: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Message {
    fn default() -> &'a Message {
        <Message as ::protobuf::Message>::default_instance()
    }
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    // .proto.Message.Type type = 1;


    pub fn get_field_type(&self) -> Message_Type {
        self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type = Message_Type::PING;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Message_Type) {
        self.field_type = v;
    }

    // string Chain = 2;


    pub fn get_Chain(&self) -> &str {
        &self.Chain
    }
    pub fn clear_Chain(&mut self) {
        self.Chain.clear();
    }

    // Param is passed by value, moved
    pub fn set_Chain(&mut self, v: ::std::string::String) {
        self.Chain = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Chain(&mut self) -> &mut ::std::string::String {
        &mut self.Chain
    }

    // Take field
    pub fn take_Chain(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.Chain, ::std::string::String::new())
    }

    // string from = 3;


    pub fn get_from(&self) -> &str {
        &self.from
    }
    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: ::std::string::String) {
        self.from = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from(&mut self) -> &mut ::std::string::String {
        &mut self.from
    }

    // Take field
    pub fn take_from(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.from, ::std::string::String::new())
    }

    // repeated string Target = 4;


    pub fn get_Target(&self) -> &[::std::string::String] {
        &self.Target
    }
    pub fn clear_Target(&mut self) {
        self.Target.clear();
    }

    // Param is passed by value, moved
    pub fn set_Target(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.Target = v;
    }

    // Mutable pointer to the field.
    pub fn mut_Target(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.Target
    }

    // Take field
    pub fn take_Target(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.Target, ::protobuf::RepeatedField::new())
    }

    // bytes Payload = 5;


    pub fn get_Payload(&self) -> &[u8] {
        &self.Payload
    }
    pub fn clear_Payload(&mut self) {
        self.Payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_Payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.Payload = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.Payload
    }

    // Take field
    pub fn take_Payload(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.Payload, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type == ::protobuf::wire_format::WireTypeVarint {self.field_type = is.read_enum()?;} else {return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));}
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.Chain)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.from)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.Target)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.Payload)?;
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
        if self.field_type != Message_Type::PING {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if !self.Chain.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.Chain);
        }
        if !self.from.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.from);
        }
        for value in &self.Target {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if !self.Payload.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.Payload);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != Message_Type::PING {
            os.write_enum(1, self.field_type.value())?;
        }
        if !self.Chain.is_empty() {
            os.write_string(2, &self.Chain)?;
        }
        if !self.from.is_empty() {
            os.write_string(3, &self.from)?;
        }
        for v in &self.Target {
            os.write_string(4, &v)?;
        };
        if !self.Payload.is_empty() {
            os.write_bytes(5, &self.Payload)?;
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

    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Message_Type>>(
                    "type",
                    |m: &Message| { &m.field_type },
                    |m: &mut Message| { &mut m.field_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "Chain",
                    |m: &Message| { &m.Chain },
                    |m: &mut Message| { &mut m.Chain },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "from",
                    |m: &Message| { &m.from },
                    |m: &mut Message| { &mut m.from },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "Target",
                    |m: &Message| { &m.Target },
                    |m: &mut Message| { &mut m.Target },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "Payload",
                    |m: &Message| { &m.Payload },
                    |m: &mut Message| { &mut m.Payload },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.field_type = Message_Type::PING;
        self.Chain.clear();
        self.from.clear();
        self.Target.clear();
        self.Payload.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Message_Type {
    PING = 0,
    PONG = 1,
    DATA = 2,
}

impl ::protobuf::ProtobufEnum for Message_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Message_Type> {
        match value {
            0 => ::std::option::Option::Some(Message_Type::PING),
            1 => ::std::option::Option::Some(Message_Type::PONG),
            2 => ::std::option::Option::Some(Message_Type::DATA),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Message_Type] = &[
            Message_Type::PING,
            Message_Type::PONG,
            Message_Type::DATA,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Message_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Message_Type {
}

impl ::std::default::Default for Message_Type {
    fn default() -> Self {
        Message_Type::PING
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\npeer.proto\x12\x05proto\x1a\x0ccommon.proto\"\xb4\x01\n\x07Message\
    \x12'\n\x04type\x18\x01\x20\x01(\x0e2\x13.proto.Message.TypeR\x04type\
    \x12\x14\n\x05Chain\x18\x02\x20\x01(\tR\x05Chain\x12\x12\n\x04from\x18\
    \x03\x20\x01(\tR\x04from\x12\x16\n\x06Target\x18\x04\x20\x03(\tR\x06Targ\
    et\x12\x18\n\x07Payload\x18\x05\x20\x01(\x0cR\x07Payload\"$\n\x04Type\
    \x12\x08\n\x04PING\x10\0\x12\x08\n\x04PONG\x10\x01\x12\x08\n\x04DATA\x10\
    \x022I\n\x08Endorser\x12=\n\x0fProcessProposal\x12\x0f.proto.Proposal\
    \x1a\x17.proto.ProposalResponse\"\021\n\x04Peer\x12)\n\x07Deliver\x12\
    \x0e.proto.Message\x1a\x0c.proto.Empty\"\028\n\x03P2P\x121\n\x07Connect\
    \x12\x0f.proto.Envelope\x1a\x0f.proto.Envelope\"\0(\x010\x01b\x06proto3\
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