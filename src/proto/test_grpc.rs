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

const METHOD_TEST_SERVICE_GET_STREAM: ::grpcio::Method<super::test::RpcRequest, super::test::RpcResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/test.TestService/GetStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TEST_SERVICE_GET_UNARY: ::grpcio::Method<super::test::RpcRequest, super::test::RpcResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/test.TestService/GetUnary",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct TestServiceClient {
    client: ::grpcio::Client,
}

impl TestServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TestServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::test::RpcRequest>, ::grpcio::ClientCStreamReceiver<super::test::RpcResponse>)> {
        self.client.client_streaming(&METHOD_TEST_SERVICE_GET_STREAM, opt)
    }

    pub fn get_stream(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::test::RpcRequest>, ::grpcio::ClientCStreamReceiver<super::test::RpcResponse>)> {
        self.get_stream_opt(::grpcio::CallOption::default())
    }

    pub fn get_unary_opt(&self, req: &super::test::RpcRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::test::RpcResponse> {
        self.client.unary_call(&METHOD_TEST_SERVICE_GET_UNARY, req, opt)
    }

    pub fn get_unary(&self, req: &super::test::RpcRequest) -> ::grpcio::Result<super::test::RpcResponse> {
        self.get_unary_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_unary_async_opt(&self, req: &super::test::RpcRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::test::RpcResponse>> {
        self.client.unary_call_async(&METHOD_TEST_SERVICE_GET_UNARY, req, opt)
    }

    pub fn get_unary_async(&self, req: &super::test::RpcRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::test::RpcResponse>> {
        self.get_unary_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait TestService {
    fn get_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::test::RpcRequest>, sink: ::grpcio::ClientStreamingSink<super::test::RpcResponse>);
    fn get_unary(&mut self, ctx: ::grpcio::RpcContext, req: super::test::RpcRequest, sink: ::grpcio::UnarySink<super::test::RpcResponse>);
}

pub fn create_test_service<S: TestService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_TEST_SERVICE_GET_STREAM, move |ctx, req, resp| {
        instance.get_stream(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_TEST_SERVICE_GET_UNARY, move |ctx, req, resp| {
        instance.get_unary(ctx, req, resp)
    });
    builder.build()
}
