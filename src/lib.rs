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
}

#[derive(Clone, Debug)]
pub struct ClientArg {
    pub ip: String,
    pub port: u16,
    pub msg_size: u32,
    pub msg_num: u32,
    pub thread_num: u32,
    pub quota_size: usize,
}
