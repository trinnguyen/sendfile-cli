use std::{env, fs::File};

use getopts::Options;
use sendfile_cli::{client::ClientStateMachine, server::ServerStateMachine};
use sendfile_cli::network::{Network, TlsTcpClient, TlsTcpServer};
use std::path::PathBuf;

extern crate getopts;

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args.first().unwrap();
    println!("{:?}", args);
    
    let mut opts = Options::new();
    opts.optflag("s", "", "run as server");
    opts.optflag("c", "", "run as client");
    opts.reqopt("p", "", "port", "PORT");
    opts.optmulti("f", "", "selected file", "FILE");

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
    let port: u32 = m.opt_get("p").unwrap().unwrap();
    if is_server {
        println!("starting server at port: {}", port);
        let listener = Network::create_tcp_listener(port);
        loop {
            println!("waiting for new TCP connection....");
            let (str, addr) = listener.accept().unwrap();
            println!("accepted new client at: {}", addr);
            let mut server = TlsTcpServer::new(str);

            // state machine
            let mut sm = ServerStateMachine::new(server.create_tls_str());
            sm.start()
        }
        

    } else if is_client {
        let files = m.opt_strs("f");
        println!("sending files: {:?}", files);
        if files.is_empty() {
            print_help(prog, &opts);
            panic!("missing input files")
        }

        // check all files are exists
        let paths: Vec<PathBuf> = files.iter().map(|f| PathBuf::from(f)).collect();
        for p in &paths {
            let f = File::open(&p).unwrap();
            if !f.metadata().unwrap().is_file() {
                panic!("invalid file: {:?}", p);
            }
        }

        println!("starting client connect to port: {}", port);
        let mut client = TlsTcpClient::connect(port);
        let mut cm = ClientStateMachine::new(client.create_tls_str(), &paths);
        cm.start()
    }
}

fn print_help(prog: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", prog);
    print!("{}", opts.usage(&brief));
}