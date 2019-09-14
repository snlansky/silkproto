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
//! Generated file from `message.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    pub message_type: Message_MessageType,
    pub correlation_id: ::std::string::String,
    pub content: ::std::vec::Vec<u8>,
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

    // .proto.Message.MessageType message_type = 1;


    pub fn get_message_type(&self) -> Message_MessageType {
        self.message_type
    }
    pub fn clear_message_type(&mut self) {
        self.message_type = Message_MessageType::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_message_type(&mut self, v: Message_MessageType) {
        self.message_type = v;
    }

    // string correlation_id = 2;


    pub fn get_correlation_id(&self) -> &str {
        &self.correlation_id
    }
    pub fn clear_correlation_id(&mut self) {
        self.correlation_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_correlation_id(&mut self, v: ::std::string::String) {
        self.correlation_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_correlation_id(&mut self) -> &mut ::std::string::String {
        &mut self.correlation_id
    }

    // Take field
    pub fn take_correlation_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.correlation_id, ::std::string::String::new())
    }

    // bytes content = 3;


    pub fn get_content(&self) -> &[u8] {
        &self.content
    }
    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.content, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type == ::protobuf::wire_format::WireTypeVarint {self.message_type = is.read_enum()?;} else {return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));}
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.correlation_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.content)?;
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
        if self.message_type != Message_MessageType::DEFAULT {
            my_size += ::protobuf::rt::enum_size(1, self.message_type);
        }
        if !self.correlation_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.correlation_id);
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.content);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.message_type != Message_MessageType::DEFAULT {
            os.write_enum(1, self.message_type.value())?;
        }
        if !self.correlation_id.is_empty() {
            os.write_string(2, &self.correlation_id)?;
        }
        if !self.content.is_empty() {
            os.write_bytes(3, &self.content)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Message_MessageType>>(
                    "message_type",
                    |m: &Message| { &m.message_type },
                    |m: &mut Message| { &mut m.message_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "correlation_id",
                    |m: &Message| { &m.correlation_id },
                    |m: &mut Message| { &mut m.correlation_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content",
                    |m: &Message| { &m.content },
                    |m: &mut Message| { &mut m.content },
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
        self.message_type = Message_MessageType::DEFAULT;
        self.correlation_id.clear();
        self.content.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Message_MessageType {
    DEFAULT = 0,
    CONSENSUS_NOTIFY_PEER_CONNECTED = 900,
    CONSENSUS_NOTIFY_PEER_DISCONNECTED = 901,
    CONSENSUS_NOTIFY_PEER_MESSAGE = 902,
    CONSENSUS_NOTIFY_BLOCK_NEW = 903,
    CONSENSUS_NOTIFY_BLOCK_VALID = 904,
    CONSENSUS_NOTIFY_BLOCK_INVALID = 905,
    CONSENSUS_NOTIFY_BLOCK_COMMIT = 906,
}

impl ::protobuf::ProtobufEnum for Message_MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Message_MessageType> {
        match value {
            0 => ::std::option::Option::Some(Message_MessageType::DEFAULT),
            900 => ::std::option::Option::Some(Message_MessageType::CONSENSUS_NOTIFY_PEER_CONNECTED),
            901 => ::std::option::Option::Some(Message_MessageType::CONSENSUS_NOTIFY_PEER_DISCONNECTED),
            902 => ::std::option::Option::Some(Message_MessageType::CONSENSUS_NOTIFY_PEER_MESSAGE),
            903 => ::std::option::Option::Some(Message_MessageType::CONSENSUS_NOTIFY_BLOCK_NEW),
            904 => ::std::option::Option::Some(Message_MessageType::CONSENSUS_NOTIFY_BLOCK_VALID),
            905 => ::std::option::Option::Some(Message_MessageType::CONSENSUS_NOTIFY_BLOCK_INVALID),
            906 => ::std::option::Option::Some(Message_MessageType::CONSENSUS_NOTIFY_BLOCK_COMMIT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Message_MessageType] = &[
            Message_MessageType::DEFAULT,
            Message_MessageType::CONSENSUS_NOTIFY_PEER_CONNECTED,
            Message_MessageType::CONSENSUS_NOTIFY_PEER_DISCONNECTED,
            Message_MessageType::CONSENSUS_NOTIFY_PEER_MESSAGE,
            Message_MessageType::CONSENSUS_NOTIFY_BLOCK_NEW,
            Message_MessageType::CONSENSUS_NOTIFY_BLOCK_VALID,
            Message_MessageType::CONSENSUS_NOTIFY_BLOCK_INVALID,
            Message_MessageType::CONSENSUS_NOTIFY_BLOCK_COMMIT,
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
                ::protobuf::reflect::EnumDescriptor::new("Message_MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Message_MessageType {
}

impl ::std::default::Default for Message_MessageType {
    fn default() -> Self {
        Message_MessageType::DEFAULT
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmessage.proto\x12\x05proto\"\xa6\x03\n\x07Message\x12=\n\x0cmessage_\
    type\x18\x01\x20\x01(\x0e2\x1a.proto.Message.MessageTypeR\x0bmessageType\
    \x12%\n\x0ecorrelation_id\x18\x02\x20\x01(\tR\rcorrelationId\x12\x18\n\
    \x07content\x18\x03\x20\x01(\x0cR\x07content\"\x9a\x02\n\x0bMessageType\
    \x12\x0b\n\x07DEFAULT\x10\0\x12$\n\x1fCONSENSUS_NOTIFY_PEER_CONNECTED\
    \x10\x84\x07\x12'\n\"CONSENSUS_NOTIFY_PEER_DISCONNECTED\x10\x85\x07\x12\
    \"\n\x1dCONSENSUS_NOTIFY_PEER_MESSAGE\x10\x86\x07\x12\x1f\n\x1aCONSENSUS\
    _NOTIFY_BLOCK_NEW\x10\x87\x07\x12!\n\x1cCONSENSUS_NOTIFY_BLOCK_VALID\x10\
    \x88\x07\x12#\n\x1eCONSENSUS_NOTIFY_BLOCK_INVALID\x10\x89\x07\x12\"\n\
    \x1dCONSENSUS_NOTIFY_BLOCK_COMMIT\x10\x8a\x072;\n\tComponent\x12.\n\x06N\
    otify\x12\x0e.proto.Message\x1a\x0e.proto.Message\"\0(\x010\x01b\x06prot\
    o3\
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
