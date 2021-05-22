use std::{env};

use getopts::Options;
use sendfile_cli::driver::{client_send_files, ServerDriver};
use std::path::{PathBuf};

extern crate getopts;

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args.first().unwrap();
    println!("{:?}", args);
    
    let mut opts = Options::new();
    opts.optflag("s", "", "run as server");
    opts.optflag("c", "", "run as client");
    opts.reqopt("p", "", "port", "PORT");
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
    if !is_server && !is_client {
        print_help(prog, &opts);
        panic!("Required -s for server or -c for client")
    }

    // start as server
    let port: u16 = m.opt_get("p").unwrap().unwrap();
    if is_server {
        println!("starting server at port: {}", port);
        let server = ServerDriver::create_server(port);
        loop {
            server.accept_conn()
        }

    } else if is_client {
        let paths: Vec<PathBuf> = m.opt_strs("f").iter().map(PathBuf::from).collect();
        if paths.is_empty() {
            print_help(prog, &opts);
            panic!("Required -f for client")
        }
        client_send_files(paths, port);
    }
}

fn print_help(prog: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", prog);
    print!("{}", opts.usage(&brief));
}