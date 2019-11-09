use clap::{App, Arg};
use grpc_bench::{server, ServerArg};

fn main() {
    // command line args
    let matches = App::new("Performance Server")
        .author("PingCAP TiKV team")
        .arg(
            Arg::with_name("Port")
                .long("port")
                .default_value("8080")
                .help("The port the server should listen on")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Case")
                .long("case")
                .help("Select a case to run")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CqNum")
                .long("cq")
                .help("CQ number")
                .default_value("2")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("MsgSize")
                .long("msg_size")
                .help("Message size to response")
                .default_value("64")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Quota")
                .long("quota_size")
                .help("Memory quota size")
                .default_value("51200")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CalculateTime")
                .long("cal_time")
                .help("Calculate time each call")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("SleepTime")
                .long("sleep_time")
                .help("Simulate IO time each call")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("max_recv_msg_len")
                .long("max_recv_msg_len")
                .help("max_recv_msg_len")
                .default_value("1048576")
                .takes_value(true),
        )
        .get_matches();

    // config initial args
    let cmd_arg = ServerArg {
        port: matches.value_of("Port").unwrap().parse().unwrap(),
        msg_size: matches.value_of("MsgSize").unwrap().parse().unwrap(),
        cq_num: matches.value_of("CqNum").unwrap().parse().unwrap(),
        quota_size: matches.value_of("Quota").unwrap().parse().unwrap(),
        cal_time: if let Some(s) = matches.value_of("CalculateTime") {
            Some(s.parse().unwrap())
        } else {
            None
        },
        sleep_time: if let Some(s) = matches.value_of("SleepTime") {
            Some(s.parse().unwrap())
        } else {
            None
        },
        max_recv_msg_len: matches
            .value_of("max_recv_msg_len")
            .unwrap()
            .parse()
            .unwrap(),
    };
    println!(
        "==== Configuration ====\n{:?}\n==== Configuration ====",
        &cmd_arg
    );

    // run cases
    match matches.value_of("Case").unwrap() {
        "test" => {
            server::run_test_server(cmd_arg);
        }
        _ => {
            println!("Please input valid name, refer to the binary files");
        }
    }
    println!("Finish all tests");
}
