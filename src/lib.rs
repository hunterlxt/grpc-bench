pub mod client;
pub mod proto;
pub mod server;
pub mod util;

pub enum RpcType {
    UNARY,
    STREAM,
}

#[derive(Clone, Debug)]
pub struct ServerArg {
    pub port: u16,
    pub msg_size: u32,
    pub cq_num: u32,
    pub quota_size: usize,
    pub cal_time: Option<u32>,
    pub sleep_time: Option<u32>,
    pub max_recv_msg_len: i32,
    pub max_concurrent_stream: i32,
}

#[derive(Clone, Debug)]
pub struct ClientArg {
    pub ip: String,
    pub port: u16,
    pub msg_size: u32,
    pub msg_num: u32,
    pub thread_num: u32,
    pub quota_size: usize,
    pub cq_num: u32,
    pub max_recv_msg_len: i32,
    pub max_concurrent_stream: i32,
}
