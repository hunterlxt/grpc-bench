use crate::proto::test::RpcRequest;
use crate::proto::test_grpc::TestServiceClient;
use crate::util::generate_bytes;
use crate::ClientArg;
use futures::*;
use grpcio::{ChannelBuilder, Environment, ResourceQuota, WriteFlags};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

pub fn unary_call(cmd: ClientArg) {
    let mut count = 0;
    let env = Arc::new(Environment::new(1));
    let addr = format!("{}:{}", cmd.ip, cmd.port);
    let bytes = generate_bytes(cmd.msg_size);
    let mut workers = vec![];
    let now = Instant::now();
    for _ in 0..cmd.thread_num {
        let bytes = bytes.clone();
        let cmd = cmd.clone();
        let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(cmd.quota_size);
        let ch = ChannelBuilder::new(env.clone())
            .set_resource_quota(quota)
            .connect(addr.as_str());
        let client = Arc::new(TestServiceClient::new(ch.clone()));
        workers.push(thread::spawn(move || {
            for _ in 0..cmd.msg_num {
                let mut req = RpcRequest::default();
                req.set_data(bytes.clone());
                client.get_unary(&req).expect("rpc call error");
            }
        }));
    }

    for worker in workers {
        worker.join().expect("join the worker thread");
    }
    println!(
        "unary_call test finished after {}",
        now.elapsed().as_secs_f64()
    );
}

pub fn stream_call(cmd: ClientArg) {
    let mut count = 0;
    let env = Arc::new(Environment::new(1));
    let addr = format!("{}:{}", cmd.ip, cmd.port);
    let ch = ChannelBuilder::new(env)
        .max_receive_message_len(1 << 10)
        .connect(addr.as_str());
    let bytes = generate_bytes(cmd.msg_size);
    let mut workers = vec![];
    let now = Instant::now();
    for _ in 0..cmd.thread_num {
        let bytes = bytes.clone();
        let cmd = cmd.clone();
        let client = Arc::new(TestServiceClient::new(ch.clone()));
        workers.push(thread::spawn(move || {
            let (mut tx, mut rx) = client.bidirect().unwrap();
            for _ in 0..cmd.msg_num {
                let mut req = RpcRequest::default();
                req.set_data(bytes.clone());
                tx = tx.send((req, WriteFlags::default())).wait().unwrap();
                match rx.into_future().wait() {
                    Ok((Some(resp), r)) => {
                        rx = r;
                    }
                    Ok((None, r)) => {
                        println!("Get none msg");
                        rx = r;
                    }
                    _ => unimplemented!(),
                }
            }
        }));
    }

    for worker in workers {
        worker.join().expect("join the worker thread");
    }
    println!(
        "stream_call test finished after {}",
        now.elapsed().as_secs_f64()
    );
}
