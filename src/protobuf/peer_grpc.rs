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

const METHOD_ENDORSER_PROCESS_PROPOSAL: ::grpcio::Method<super::common::Proposal, super::common::ProposalResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/proto.Endorser/ProcessProposal",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct EndorserClient {
    client: ::grpcio::Client,
}

impl EndorserClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        EndorserClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn process_proposal_opt(&self, req: &super::common::Proposal, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::ProposalResponse> {
        self.client.unary_call(&METHOD_ENDORSER_PROCESS_PROPOSAL, req, opt)
    }

    pub fn process_proposal(&self, req: &super::common::Proposal) -> ::grpcio::Result<super::common::ProposalResponse> {
        self.process_proposal_opt(req, ::grpcio::CallOption::default())
    }

    pub fn process_proposal_async_opt(&self, req: &super::common::Proposal, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::ProposalResponse>> {
        self.client.unary_call_async(&METHOD_ENDORSER_PROCESS_PROPOSAL, req, opt)
    }

    pub fn process_proposal_async(&self, req: &super::common::Proposal) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::ProposalResponse>> {
        self.process_proposal_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Endorser {
    fn process_proposal(&mut self, ctx: ::grpcio::RpcContext, req: super::common::Proposal, sink: ::grpcio::UnarySink<super::common::ProposalResponse>);
}

pub fn create_endorser<S: Endorser + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ENDORSER_PROCESS_PROPOSAL, move |ctx, req, resp| {
        instance.process_proposal(ctx, req, resp)
    });
    builder.build()
}

const METHOD_PEER_DELIVER: ::grpcio::Method<super::peer::Message, super::common::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/proto.Peer/Deliver",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct PeerClient {
    client: ::grpcio::Client,
}

impl PeerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        PeerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn deliver_opt(&self, req: &super::peer::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::Empty> {
        self.client.unary_call(&METHOD_PEER_DELIVER, req, opt)
    }

    pub fn deliver(&self, req: &super::peer::Message) -> ::grpcio::Result<super::common::Empty> {
        self.deliver_opt(req, ::grpcio::CallOption::default())
    }

    pub fn deliver_async_opt(&self, req: &super::peer::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Empty>> {
        self.client.unary_call_async(&METHOD_PEER_DELIVER, req, opt)
    }

    pub fn deliver_async(&self, req: &super::peer::Message) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Empty>> {
        self.deliver_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Peer {
    fn deliver(&mut self, ctx: ::grpcio::RpcContext, req: super::peer::Message, sink: ::grpcio::UnarySink<super::common::Empty>);
}

pub fn create_peer<S: Peer + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PEER_DELIVER, move |ctx, req, resp| {
        instance.deliver(ctx, req, resp)
    });
    builder.build()
}

const METHOD_P2_P_CONNECT: ::grpcio::Method<super::common::Envelope, super::common::Envelope> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/proto.P2P/Connect",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct P2PClient {
    client: ::grpcio::Client,
}

impl P2PClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        P2PClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn connect_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::common::Envelope>, ::grpcio::ClientDuplexReceiver<super::common::Envelope>)> {
        self.client.duplex_streaming(&METHOD_P2_P_CONNECT, opt)
    }

    pub fn connect(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::common::Envelope>, ::grpcio::ClientDuplexReceiver<super::common::Envelope>)> {
        self.connect_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait P2P {
    fn connect(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::common::Envelope>, sink: ::grpcio::DuplexSink<super::common::Envelope>);
}

pub fn create_p2_p<S: P2P + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_P2_P_CONNECT, move |ctx, req, resp| {
        instance.connect(ctx, req, resp)
    });
    builder.build()
}
