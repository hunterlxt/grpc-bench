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
        .get_matches();

    // config initial args
    let mut cmd_arg = ServerArg {
        port: matches.value_of("Port").unwrap().parse().unwrap(),
        msg_size: matches.value_of("MsgSize").unwrap().parse().unwrap(),
        cq_num: matches.value_of("CqNum").unwrap().parse().unwrap(),
        quota_size: matches.value_of("Quota").unwrap().parse().unwrap(),
    };
    println!(
        "==== Configuration ====\n{:?}\n==== Start Case ====",
        &cmd_arg
    );

    // run cases
    match matches.value_of("Case").unwrap() {
        "ping_pong" => {
            server::echo::ping_pong(cmd_arg);
        }
        _ => {
            println!("Please input valid name, refer to the binary files");
        }
    }
    println!("Finish all tests");
}
