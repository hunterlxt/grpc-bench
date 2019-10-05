use clap::{App, Arg};
use grpc_bench::client;
use grpc_bench::ClientArg;

fn main() {
    // command line args
    let matches = App::new("Performance Client")
        .author("PingCAP TiKV team")
        .arg(
            Arg::with_name("IP")
                .long("ip")
                .default_value("0.0.0.0")
                .help("The ip address to connect")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Port")
                .long("port")
                .default_value("8080")
                .help("The port to connect")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Case")
                .long("case")
                .help("Select a case to run")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // config initial args
    let mut cmd_arg = ClientArg {
        ip: matches.value_of("IP").unwrap().to_owned(),
        port: matches.value_of("Port").unwrap().parse().unwrap(),
        req_size: 1,
        msg_num: 1,
        thread_num: 1,
    };

    // run cases (Required!)
    match matches.value_of("Case").unwrap() {
        "ping_pong_64B_2000" => {
            cmd_arg.req_size = 64;
            cmd_arg.msg_num = 2000;
            client::echo::ping_pong(cmd_arg);
        }
        "ping_pong_64B_unlimited" => {
            cmd_arg.req_size = 64;
            cmd_arg.msg_num = u32::max_value();
            client::echo::ping_pong(cmd_arg);
        }
        "ping_pong_64B_unlimited_4threads" => {
            cmd_arg.req_size = 64;
            cmd_arg.msg_num = u32::max_value();
            cmd_arg.thread_num = 4;
            client::echo::ping_pong(cmd_arg);
        }
        "ping_pong_1MB_2000" => {
            cmd_arg.req_size = 1024 * 1024;
            cmd_arg.msg_num = 2000;
            client::echo::ping_pong(cmd_arg);
        }
        "ping_pong_1MB_unlimited" => {
            cmd_arg.req_size = 1024 * 1024;
            cmd_arg.msg_num = u32::max_value();
            client::echo::ping_pong(cmd_arg);
        }
        "ping_pong_1MB_unlimited_4threads" => {
            cmd_arg.req_size = 1024 * 1024;
            cmd_arg.msg_num = u32::max_value();
            cmd_arg.thread_num = 4;
            client::echo::ping_pong(cmd_arg);
        }
        _ => {
            println!("Please input valid name, refer to the file in src/bin/");
        }
    }
    println!("Finish all tests");
}
