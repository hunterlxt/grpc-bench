use crate::proto::test::{RpcRequest, RpcResponse};
use crate::proto::test_grpc::{create_test_service, TestService};
use crate::util::generate_bytes;
use crate::ServerArg;
use futures::Stream;
use futures::*;
use grpcio::*;
use grpcio::{
    ChannelBuilder, ClientStreamingSink, DuplexSink, Environment, Error, RequestStream,
    ResourceQuota, RpcContext, ServerBuilder, ServerStreamingSink, UnarySink,
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
        std::thread::sleep(std::time::Duration::from_secs(3));
        let f = sink.success(resp).map_err(|_| {});
        ctx.spawn(f);
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
        let f = stream
            .fold(0, move |mut sum, mut req| {
                let _msg = req.get_data();
                sum += 1;
                println!("sum:{}", sum);
                Ok(sum) as Result<_>
            })
            .and_then(move |sum| {
                let mut resp = RpcResponse::default();
                resp.set_data(generate_bytes(sum));
                sink.send((resp, WriteFlags::default()))
            })
            .map(|_| ())
            .map_err(|e| {});
        //     .sink_map_err(|e| Error::GoogleAuthenticationFailed)
        //     .send_all(
        //         stream
        //             .map_err(|e| Error::GoogleAuthenticationFailed)
        //             .and_then(move |req| {
        //                 let msg = req.get_data();
        //                 let mut resp = RpcResponse::default();
        //                 resp.set_data(msg.to_vec());
        //                 Ok(Some((resp, WriteFlags::default())))
        //             })
        //             .filter_map(|o| o),
        //     )
        //     .map(|_| ())
        //     .map_err(|e| {});
        ctx.spawn(f);
    }
}

pub fn ping_pong(cmd: ServerArg) {
    let env = Arc::new(Environment::new(cmd.cq_num as _));
    let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(cmd.quota_size);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);
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
