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

const METHOD_ATOMIC_BROADCAST_BROADCAST: ::grpcio::Method<super::common::ChannelHeader, super::consensus::BroadcastResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/proto.AtomicBroadcast/Broadcast",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ATOMIC_BROADCAST_DELIVER: ::grpcio::Method<super::common::Envelope, super::consensus::DeliverResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/proto.AtomicBroadcast/Deliver",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AtomicBroadcastClient {
    client: ::grpcio::Client,
}

impl AtomicBroadcastClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AtomicBroadcastClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn broadcast_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::common::ChannelHeader>, ::grpcio::ClientDuplexReceiver<super::consensus::BroadcastResponse>)> {
        self.client.duplex_streaming(&METHOD_ATOMIC_BROADCAST_BROADCAST, opt)
    }

    pub fn broadcast(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::common::ChannelHeader>, ::grpcio::ClientDuplexReceiver<super::consensus::BroadcastResponse>)> {
        self.broadcast_opt(::grpcio::CallOption::default())
    }

    pub fn deliver_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::common::Envelope>, ::grpcio::ClientDuplexReceiver<super::consensus::DeliverResponse>)> {
        self.client.duplex_streaming(&METHOD_ATOMIC_BROADCAST_DELIVER, opt)
    }

    pub fn deliver(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::common::Envelope>, ::grpcio::ClientDuplexReceiver<super::consensus::DeliverResponse>)> {
        self.deliver_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AtomicBroadcast {
    fn broadcast(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::common::ChannelHeader>, sink: ::grpcio::DuplexSink<super::consensus::BroadcastResponse>);
    fn deliver(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::common::Envelope>, sink: ::grpcio::DuplexSink<super::consensus::DeliverResponse>);
}

pub fn create_atomic_broadcast<S: AtomicBroadcast + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_ATOMIC_BROADCAST_BROADCAST, move |ctx, req, resp| {
        instance.broadcast(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_ATOMIC_BROADCAST_DELIVER, move |ctx, req, resp| {
        instance.deliver(ctx, req, resp)
    });
    builder.build()
}
