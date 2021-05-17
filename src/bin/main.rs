use std::env;

use getopts::Options;
use sendfile_cli::{client::ClientStateMachine, common::FileInfo, network::{TcpClient, TcpServer}, server::ServerStateMachine};

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
        let listener = TcpServer::create_tcp_listener(port);
        loop {
            println!("waiting for new TCP connection....");
            let (str, addr) = listener.accept().unwrap();
            println!("accepted new client at: {}", addr);
            let mut server = TcpServer::new(str);

            // state machine
            let mut sm = ServerStateMachine::new(server.get_tls_str());
            sm.start()
        }
        

    } else if is_client {
        let files = m.opt_strs("f");
        println!("sending files: {:?}", files);
        let infos: Vec<FileInfo> = files.iter().map(|p| FileInfo::from_path(p)).collect();
        println!("file info: {:?}", infos);
        if files.is_empty() {
            print_help(prog, &opts);
            panic!("missing input files")
        }

        println!("starting client connect to port: {}", port);
        let mut client = TcpClient::connect(port);
        let mut cm = ClientStateMachine::new(client.create_tls_str(), infos);
        cm.start()
    }
}

fn print_help(prog: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", prog);
    print!("{}", opts.usage(&brief));
}