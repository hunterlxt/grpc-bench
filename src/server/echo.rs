use crate::proto::test::{RpcRequest, RpcResponse};
use crate::proto::test_grpc::{create_test_service, TestService};
use crate::ServerArg;
use grpcio::{
    ClientStreamingSink, DuplexSink, Environment, RequestStream, RpcContext, ServerBuilder,
    ServerStreamingSink, UnarySink,
};
use std::sync::Arc;
use std::thread;
use std::time;

#[derive(Clone)]
struct EchoService {
    cmd: ServerArg,
}

impl TestService for EchoService {
    fn get_unary(&mut self, ctx: RpcContext, req: RpcRequest, sink: UnarySink<RpcResponse>) {
        let msg = req.get_data();
        let mut resp = RpcResponse::default();
        resp.set_data(msg.to_vec());
        sink.success(resp);
    }

    fn get_stream(
        &mut self,
        ctx: RpcContext,
        req: RpcRequest,
        sink: ServerStreamingSink<RpcResponse>,
    ) {
    }

    fn send_stream(
        &mut self,
        ctx: RpcContext,
        req: RequestStream<RpcRequest>,
        sink: ClientStreamingSink<RpcResponse>,
    ) {
    }

    fn bidirect(
        &mut self,
        ctx: RpcContext,
        stream: RequestStream<RpcRequest>,
        sink: DuplexSink<RpcResponse>,
    ) {
    }
}

pub fn ping_pong(cmd: ServerArg) {
    let env = Arc::new(Environment::new(cmd.cq_num as _));
    let service = create_test_service(EchoService { cmd: cmd.clone() });
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", cmd.port)
        .build()
        .unwrap();
    server.start();

    thread::sleep(time::Duration::from_secs(24 * 3600));
}
