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

const METHOD_NETWORK_BROADCAST: ::grpcio::Method<super::common::Envelope, super::common::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/proto.Network/Broadcast",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct NetworkClient {
    client: ::grpcio::Client,
}

impl NetworkClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        NetworkClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn broadcast_opt(&self, req: &super::common::Envelope, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::Empty> {
        self.client.unary_call(&METHOD_NETWORK_BROADCAST, req, opt)
    }

    pub fn broadcast(&self, req: &super::common::Envelope) -> ::grpcio::Result<super::common::Empty> {
        self.broadcast_opt(req, ::grpcio::CallOption::default())
    }

    pub fn broadcast_async_opt(&self, req: &super::common::Envelope, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Empty>> {
        self.client.unary_call_async(&METHOD_NETWORK_BROADCAST, req, opt)
    }

    pub fn broadcast_async(&self, req: &super::common::Envelope) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Empty>> {
        self.broadcast_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Network {
    fn broadcast(&mut self, ctx: ::grpcio::RpcContext, req: super::common::Envelope, sink: ::grpcio::UnarySink<super::common::Empty>);
}

pub fn create_network<S: Network + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NETWORK_BROADCAST, move |ctx, req, resp| {
        instance.broadcast(ctx, req, resp)
    });
    builder.build()
}
