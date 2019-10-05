use crate::proto::test::RpcRequest;
use crate::proto::test_grpc::TestServiceClient;
use crate::util::generate_bytes;
use crate::ClientArg;
use grpcio::{ChannelBuilder, Environment};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

pub fn ping_pong(cmd: ClientArg) {
    let bytes = generate_bytes(cmd.req_size);
    let mut workers = vec![];
    let now = Instant::now();
    for _ in 0..cmd.thread_num {
        let bytes = bytes.clone();
        let cmd = cmd.clone();
        workers.push(thread::spawn(move || {
            let mut count = 0;
            let env = Arc::new(Environment::new(1));
            let addr = format!("{}:{}", cmd.ip, cmd.port);
            let ch = ChannelBuilder::new(env).connect(addr.as_str());
            let client = Arc::new(TestServiceClient::new(ch));
            loop {
                let mut req = RpcRequest::default();
                req.set_data(bytes.clone());
                client.get_unary(&req).expect("rpc call error");
                count += 1;
                if count >= cmd.msg_num {
                    break;
                }
            }
        }));
    }

    for worker in workers {
        worker.join().expect("join the worker thread");
    }
    println!("test finished after {}", now.elapsed().as_secs_f64());
}
