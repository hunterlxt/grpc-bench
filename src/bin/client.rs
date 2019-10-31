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
        .arg(
            Arg::with_name("MsgSize")
                .long("msg_size")
                .help("Message size to send")
                .default_value("64")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("MsgNum")
                .long("msg_num")
                .help("Unary call number each thread")
                .default_value("1000")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ThreadNum")
                .long("thread_num")
                .help("Thread num")
                .default_value("1")
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
    let cmd_arg = ClientArg {
        ip: matches.value_of("IP").unwrap().to_owned(),
        port: matches.value_of("Port").unwrap().parse().unwrap(),
        msg_size: matches.value_of("MsgSize").unwrap().parse().unwrap(),
        msg_num: matches.value_of("MsgNum").unwrap().parse().unwrap(),
        thread_num: matches.value_of("ThreadNum").unwrap().parse().unwrap(),
    };
    println!(
        "==== Configuration ====\n{:?}\n==== Start Case ====",
        &cmd_arg
    );

    // run cases (Required!)
    match matches.value_of("Case").unwrap() {
        "unary_call" => {
            client::echo::unary_call(cmd_arg);
        }
        "stream_call" => {
            client::echo::stream_call(cmd_arg);
        }
        _ => {
            println!("Please input valid name, refer to the file in src/bin/");
        }
    }
    println!("Finish all tests");
}
