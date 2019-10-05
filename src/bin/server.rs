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
        .get_matches();

    // config initial args
    let mut cmd_arg = ServerArg {
        port: matches.value_of("Port").unwrap().parse().unwrap(),
        resp_size: 1,
        cq_num: 1,
    };

    // run cases
    match matches.value_of("Case").unwrap() {
        "ping_pong_2cq" => {
            cmd_arg.cq_num = 2;
            server::echo::ping_pong(cmd_arg);
        }
        _ => {
            println!("Please input valid name, refer to the file in src/bin/");
        }
    }
    println!("Finish all tests");
}
