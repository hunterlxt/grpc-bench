use crate::proto::test::{RpcRequest, RpcResponse};
use crate::proto::test_grpc::{create_test_service, TestService};
use crate::ServerArg;
use grpcio::{
    ClientStreamingSink, Environment, RequestStream, RpcContext, ServerBuilder, UnarySink,
};
use std::sync::Arc;
use std::thread;
use std::time;

#[derive(Clone)]
struct EchoService;

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
        stream: RequestStream<RpcRequest>,
        sink: ClientStreamingSink<RpcResponse>,
    ) {
    }
}

/// 2 completion queues
pub fn ping_pong(cmd: ServerArg) {
    let env = Arc::new(Environment::new(2));
    let service = create_test_service(EchoService {});
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", cmd.port)
        .build()
        .unwrap();
    server.start();

    thread::sleep(time::Duration::from_secs(12 * 3600));
}
