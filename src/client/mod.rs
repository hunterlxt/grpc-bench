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
    let req_msg = generate_bytes(cmd.msg_size);
    let mut workers = vec![];
    let now = Instant::now();
    // Start core code
    let addr = format!("{}:{}", cmd.ip, cmd.port);
    let env = Arc::new(Environment::new(cmd.cq_num as usize));
    for _ in 0..cmd.thread_num {
        let req_msg = req_msg.clone();
        let cmd = cmd.clone();
        let env = env.clone();
        let addr = addr.clone();
        workers.push(thread::spawn(move || {
            let quota = ResourceQuota::new(Some("ClientQuota")).resize_memory(cmd.quota_size);
            let ch = ChannelBuilder::new(env)
                .set_resource_quota(quota)
                .connect(addr.as_str());
            let client = TestServiceClient::new(ch);
            for _ in 0..cmd.msg_num {
                let mut req = RpcRequest::default();
                req.set_data(req_msg.clone());
                client.get_unary(&req).expect("rpc call error");
            }
        }));
    }
    for worker in workers {
        worker.join().expect("join the worker thread");
    }
    // End core code
    println!(
        "unary_call test finished after {}",
        now.elapsed().as_secs_f64()
    );
}

pub fn send_stream(cmd: ClientArg) {
    let req_msg = generate_bytes(cmd.msg_size);
    let mut workers = vec![];
    let now = Instant::now();
    // Start core code
    let addr = format!("{}:{}", cmd.ip, cmd.port);
    let env = Arc::new(Environment::new(cmd.cq_num as usize));
    for _ in 0..cmd.thread_num {
        let req_msg = req_msg.clone();
        let cmd = cmd.clone();
        let env = env.clone();
        let addr = addr.clone();
        workers.push(thread::spawn(move || {
            let quota = ResourceQuota::new(Some("ClientQuota")).resize_memory(cmd.quota_size);
            let ch = ChannelBuilder::new(env)
                .max_concurrent_stream(cmd.max_concurrent_stream)
                .max_receive_message_len(cmd.max_recv_msg_len)
                .set_resource_quota(quota)
                .connect(addr.as_str());
            let client = TestServiceClient::new(ch);
            let (mut tx, rx) = client.send_stream().unwrap();
            for _ in 0..cmd.msg_num {
                let mut req = RpcRequest::default();
                req.set_data(req_msg.clone());
                tx = tx.send((req, WriteFlags::default())).wait().unwrap();
            }
            future::poll_fn(|| tx.close()).wait().unwrap();
            let resp = rx.wait().unwrap();
            assert_eq!(resp.get_data().len(), cmd.msg_num as _);
        }));
    }
    for worker in workers {
        worker.join().expect("join the worker thread");
    }
    // End core code
    println!(
        "send_stream test finished after {}",
        now.elapsed().as_secs_f64()
    );
}

pub fn bidirect_stream(cmd: ClientArg) {
    let req_msg = generate_bytes(cmd.msg_size);
    let mut workers = vec![];
    let now = Instant::now();
    // Start core code
    let addr = format!("{}:{}", cmd.ip, cmd.port);
    let env = Arc::new(Environment::new(cmd.cq_num as usize));
    for _ in 0..cmd.thread_num {
        let req_msg = req_msg.clone();
        let cmd = cmd.clone();
        let env = env.clone();
        let addr = addr.clone();
        workers.push(thread::spawn(move || {
            let quota = ResourceQuota::new(Some("ClientQuota")).resize_memory(cmd.quota_size);
            let ch = ChannelBuilder::new(env)
                .max_concurrent_stream(cmd.max_concurrent_stream)
                .max_receive_message_len(cmd.max_recv_msg_len)
                .set_resource_quota(quota)
                .connect(addr.as_str());
            let client = TestServiceClient::new(ch);
            let (mut tx, rx) = client.bidirect().unwrap();
            for _ in 0..cmd.msg_num {
                let mut req = RpcRequest::default();
                req.set_data(req_msg.clone());
                tx = tx.send((req, WriteFlags::default())).wait().unwrap();
            }
            future::poll_fn(|| tx.close()).wait().unwrap();
            match rx.into_future().wait() {
                Ok((Some(resp), _r)) => {
                    assert_eq!(cmd.msg_num as usize, resp.get_data().len());
                }
                Ok((None, _r)) => {
                    panic!("Get none msg!");
                }
                Err((e, _r)) => {
                    println!("{:?}", e);
                    panic!("Received an error");
                }
            }
        }));
    }
    for worker in workers {
        worker.join().expect("join the worker thread");
    }
    // End core code
    println!(
        "bidirect_stream test finished after {}",
        now.elapsed().as_secs_f64()
    );
}
