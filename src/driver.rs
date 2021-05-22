use crate::tls::{TlsTcpServer, TlsTcpClient};
use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr};
use crate::server::ServerStateMachine;
use std::path::{PathBuf};
use std::fs::File;
use crate::client::ClientStateMachine;

pub struct ServerDriver {
    listener: TcpListener
}

impl ServerDriver {
    pub fn create_server(port: u16) -> Self {
        // start TCP
        let listener = TcpListener::bind(create_localhost_addr(port)).expect("cannot start TCP");
        ServerDriver {
            listener
        }
    }

    pub fn accept_conn(&self) {
        println!("waiting for new TCP connection....");
        let (str, addr) = self.listener.accept().unwrap();
        println!("accepted new client at: {}", addr);
        let mut server = TlsTcpServer::new(str);

        // state machine
        let mut sm = ServerStateMachine::new(server.create_tls_str());
        sm.start()
    }
}

pub fn client_send_files(paths: Vec<PathBuf>, port: u16) {
    println!("sending files: {:?}", paths);

    // check all files are exists
    for p in &paths {
        let f = File::open(&p).unwrap();
        if !f.metadata().unwrap().is_file() {
            panic!("invalid file: {:?}", p);
        }
    }

    println!("starting client connect to port: {}", port);
    let mut client = TlsTcpClient::connect(create_localhost_addr(port));
    let mut cm = ClientStateMachine::new(client.create_tls_str(), &paths);
    cm.start()
}

fn create_localhost_addr(port: u16) -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
}