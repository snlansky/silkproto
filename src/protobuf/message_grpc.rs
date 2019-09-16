// This file is generated. Do not edit
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

const METHOD_COMPONENT_REGISTER: ::grpcio::Method<super::message::Message, super::message::Message> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/proto.Component/Register",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ComponentClient {
    client: ::grpcio::Client,
}

impl ComponentClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ComponentClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn register_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::message::Message>, ::grpcio::ClientDuplexReceiver<super::message::Message>)> {
        self.client.duplex_streaming(&METHOD_COMPONENT_REGISTER, opt)
    }

    pub fn register(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::message::Message>, ::grpcio::ClientDuplexReceiver<super::message::Message>)> {
        self.register_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Component {
    fn register(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::message::Message>, sink: ::grpcio::DuplexSink<super::message::Message>);
}

pub fn create_component<S: Component + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_COMPONENT_REGISTER, move |ctx, req, resp| {
        instance.register(ctx, req, resp)
    });
    builder.build()
}
