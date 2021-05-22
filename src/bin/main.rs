use std::env;

use getopts::{HasArg, Occur, Options};
use sendfile_cli::driver::{client_send_files, ServerDriver};
use std::path::PathBuf;

extern crate getopts;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let prog = args.first().unwrap();

    let mut opts = Options::new();
    opts.opt(
        "s",
        "server",
        "start server with port (example: -s 8080)",
        "PORT",
        HasArg::Yes,
        Occur::Optional,
    );
    opts.opt(
        "c",
        "client",
        "connect to server",
        "SERVER_ADDRESS (example: -c 127.0.0.1:8080)",
        HasArg::Yes,
        Occur::Optional,
    );
    opts.optmulti("f", "", "selected file (for client)", "FILE");

    // parse
    let m = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            print_help(prog, &opts);
            panic!("{}", e)
        }
    };

    // print
    let is_server = m.opt_present("s");
    let is_client = m.opt_present("c");
    match (is_server, is_client) {
        (false, false) => {
            print_help(prog, &opts);
            panic!("Required -s for server or -c for client")
        }
        (true, _) => {
            let port: u16 = m.opt_get("s").unwrap().unwrap();
            let server = ServerDriver::create_server(port);
            loop {
                server.accept_conn()
            }
        }
        (_, true) => {
            let paths: Vec<PathBuf> = m.opt_strs("f").iter().map(PathBuf::from).collect();
            if paths.is_empty() {
                print_help(prog, &opts);
                panic!("Required -f for client")
            }
            let addr: String = m.opt_get("c").unwrap().unwrap();
            client_send_files(paths, addr);
        }
    }
}

fn print_help(prog: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", prog);
    print!("{}", opts.usage(&brief));
}
