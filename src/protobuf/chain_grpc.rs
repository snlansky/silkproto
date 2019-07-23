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

const METHOD_CHAIN_FIND_OR_STORE: ::grpcio::Method<super::common::Proposal, super::chain::ResponseHash> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/proto.Chain/FindOrStore",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CHAIN_FIND_LAST_BLOCK: ::grpcio::Method<super::common::Empty, super::block::Block> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/proto.Chain/FindLastBlock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CHAIN_EXECUTOR_QUERY: ::grpcio::Method<super::common::Proposal, super::common::ProposalResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/proto.Chain/ExecutorQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ChainClient {
    client: ::grpcio::Client,
}

impl ChainClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ChainClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn find_or_store_opt(&self, req: &super::common::Proposal, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::chain::ResponseHash> {
        self.client.unary_call(&METHOD_CHAIN_FIND_OR_STORE, req, opt)
    }

    pub fn find_or_store(&self, req: &super::common::Proposal) -> ::grpcio::Result<super::chain::ResponseHash> {
        self.find_or_store_opt(req, ::grpcio::CallOption::default())
    }

    pub fn find_or_store_async_opt(&self, req: &super::common::Proposal, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::chain::ResponseHash>> {
        self.client.unary_call_async(&METHOD_CHAIN_FIND_OR_STORE, req, opt)
    }

    pub fn find_or_store_async(&self, req: &super::common::Proposal) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::chain::ResponseHash>> {
        self.find_or_store_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn find_last_block_opt(&self, req: &super::common::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::block::Block> {
        self.client.unary_call(&METHOD_CHAIN_FIND_LAST_BLOCK, req, opt)
    }

    pub fn find_last_block(&self, req: &super::common::Empty) -> ::grpcio::Result<super::block::Block> {
        self.find_last_block_opt(req, ::grpcio::CallOption::default())
    }

    pub fn find_last_block_async_opt(&self, req: &super::common::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::block::Block>> {
        self.client.unary_call_async(&METHOD_CHAIN_FIND_LAST_BLOCK, req, opt)
    }

    pub fn find_last_block_async(&self, req: &super::common::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::block::Block>> {
        self.find_last_block_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn executor_query_opt(&self, req: &super::common::Proposal, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::ProposalResponse> {
        self.client.unary_call(&METHOD_CHAIN_EXECUTOR_QUERY, req, opt)
    }

    pub fn executor_query(&self, req: &super::common::Proposal) -> ::grpcio::Result<super::common::ProposalResponse> {
        self.executor_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn executor_query_async_opt(&self, req: &super::common::Proposal, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::ProposalResponse>> {
        self.client.unary_call_async(&METHOD_CHAIN_EXECUTOR_QUERY, req, opt)
    }

    pub fn executor_query_async(&self, req: &super::common::Proposal) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::ProposalResponse>> {
        self.executor_query_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Chain {
    fn find_or_store(&mut self, ctx: ::grpcio::RpcContext, req: super::common::Proposal, sink: ::grpcio::UnarySink<super::chain::ResponseHash>);
    fn find_last_block(&mut self, ctx: ::grpcio::RpcContext, req: super::common::Empty, sink: ::grpcio::UnarySink<super::block::Block>);
    fn executor_query(&mut self, ctx: ::grpcio::RpcContext, req: super::common::Proposal, sink: ::grpcio::UnarySink<super::common::ProposalResponse>);
}

pub fn create_chain<S: Chain + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CHAIN_FIND_OR_STORE, move |ctx, req, resp| {
        instance.find_or_store(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CHAIN_FIND_LAST_BLOCK, move |ctx, req, resp| {
        instance.find_last_block(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CHAIN_EXECUTOR_QUERY, move |ctx, req, resp| {
        instance.executor_query(ctx, req, resp)
    });
    builder.build()
}
