use crate::proto::test::{RpcRequest, RpcResponse};
use crate::proto::test_grpc::{create_test_service, TestService};
use crate::util::generate_bytes;
use crate::ServerArg;
use futures::*;

use grpcio::{
    ChannelBuilder, ClientStreamingSink, DuplexSink, Environment, RequestStream, ResourceQuota,
    Result, RpcContext, ServerBuilder, ServerStreamingSink, UnarySink, WriteFlags,
};
use std::sync::Arc;
use std::thread;
use std::time;

#[derive(Clone)]
struct EchoService {
    cmd: ServerArg,
}

fn calculate_for_a_while(cmd: ServerArg) {
    if let Some(time) = cmd.cal_time {
        for _ in 0..time {}
    }
}

fn sleep_for_a_while(cmd: ServerArg) {
    if let Some(time) = cmd.sleep_time {
        std::thread::sleep(std::time::Duration::from_secs(u64::from(time)));
    }
}

impl TestService for EchoService {
    fn get_unary(&mut self, ctx: RpcContext, req: RpcRequest, sink: UnarySink<RpcResponse>) {
        calculate_for_a_while(self.cmd.clone());
        sleep_for_a_while(self.cmd.clone());
        let mut resp = RpcResponse::default();
        resp.set_data(req.get_data().to_vec());
        let f = sink.success(resp).map_err(|_| {});
        ctx.spawn(f);
    }

    fn get_stream(
        &mut self,
        _ctx: RpcContext,
        _req: RpcRequest,
        _sink: ServerStreamingSink<RpcResponse>,
    ) {
    }

    fn send_stream(
        &mut self,
        ctx: RpcContext,
        stream: RequestStream<RpcRequest>,
        sink: ClientStreamingSink<RpcResponse>,
    ) {
        calculate_for_a_while(self.cmd.clone());
        sleep_for_a_while(self.cmd.clone());
        let f = stream
            .fold(0, move |mut num, _req| {
                num += 1;
                Ok(num) as Result<_>
            })
            .and_then(move |num| {
                let mut resp = RpcResponse::default();
                resp.set_data(generate_bytes(num));
                sink.success(resp)
            })
            .map_err(|e| println!("{:?}", e));
        ctx.spawn(f);
    }

    fn bidirect(
        &mut self,
        ctx: RpcContext,
        stream: RequestStream<RpcRequest>,
        sink: DuplexSink<RpcResponse>,
    ) {
        let f = stream
            .fold(0, move |mut num, _req| {
                num += 1;
                Ok(num) as Result<_>
            })
            .and_then(move |num| {
                let mut resp = RpcResponse::default();
                resp.set_data(generate_bytes(num));
                sink.send((resp, WriteFlags::default()))
            })
            .map(|mut sink| future::poll_fn(move || sink.close().map_err(|e| println!("{:?}", e))))
            .map_err(|e| println!("{:?}", e))
            .map(|_| {});
        ctx.spawn(f);
    }
}

pub fn run_test_server(cmd: ServerArg) {
    let env = Arc::new(Environment::new(cmd.cq_num as _));
    let quota = ResourceQuota::new(Some("TestServerQuota")).resize_memory(cmd.quota_size);
    let ch_builder = ChannelBuilder::new(env.clone())
        .max_concurrent_stream(cmd.max_concurrent_stream)
        .max_receive_message_len(cmd.max_recv_msg_len)
        .set_resource_quota(quota);
    let service = create_test_service(EchoService { cmd: cmd.clone() });
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", cmd.port)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server.start();

    thread::sleep(time::Duration::from_secs(24 * 3600));
}
