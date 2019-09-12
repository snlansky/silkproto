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
    ty: ::grpcio::MethodType::Unary,
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

    pub fn broadcast_opt(&self, req: &super::common::ChannelHeader, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::consensus::BroadcastResponse> {
        self.client.unary_call(&METHOD_ATOMIC_BROADCAST_BROADCAST, req, opt)
    }

    pub fn broadcast(&self, req: &super::common::ChannelHeader) -> ::grpcio::Result<super::consensus::BroadcastResponse> {
        self.broadcast_opt(req, ::grpcio::CallOption::default())
    }

    pub fn broadcast_async_opt(&self, req: &super::common::ChannelHeader, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::consensus::BroadcastResponse>> {
        self.client.unary_call_async(&METHOD_ATOMIC_BROADCAST_BROADCAST, req, opt)
    }

    pub fn broadcast_async(&self, req: &super::common::ChannelHeader) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::consensus::BroadcastResponse>> {
        self.broadcast_async_opt(req, ::grpcio::CallOption::default())
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
    fn broadcast(&mut self, ctx: ::grpcio::RpcContext, req: super::common::ChannelHeader, sink: ::grpcio::UnarySink<super::consensus::BroadcastResponse>);
    fn deliver(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::common::Envelope>, sink: ::grpcio::DuplexSink<super::consensus::DeliverResponse>);
}

pub fn create_atomic_broadcast<S: AtomicBroadcast + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ATOMIC_BROADCAST_BROADCAST, move |ctx, req, resp| {
        instance.broadcast(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_ATOMIC_BROADCAST_DELIVER, move |ctx, req, resp| {
        instance.deliver(ctx, req, resp)
    });
    builder.build()
}

const METHOD_CONSENSUS_EXCHANGE: ::grpcio::Method<super::common::Envelope, super::common::Envelope> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/proto.Consensus/Exchange",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ConsensusClient {
    client: ::grpcio::Client,
}

impl ConsensusClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ConsensusClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn exchange_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::common::Envelope>, ::grpcio::ClientDuplexReceiver<super::common::Envelope>)> {
        self.client.duplex_streaming(&METHOD_CONSENSUS_EXCHANGE, opt)
    }

    pub fn exchange(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::common::Envelope>, ::grpcio::ClientDuplexReceiver<super::common::Envelope>)> {
        self.exchange_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Consensus {
    fn exchange(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::common::Envelope>, sink: ::grpcio::DuplexSink<super::common::Envelope>);
}

pub fn create_consensus<S: Consensus + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_CONSENSUS_EXCHANGE, move |ctx, req, resp| {
        instance.exchange(ctx, req, resp)
    });
    builder.build()
}
