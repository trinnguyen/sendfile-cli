use crate::client::ClientStateMachine;
use crate::server::ServerStateMachine;
use crate::tls::{TlsTcpClient, TlsTcpServer};
use log::info;
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::path::PathBuf;

pub struct ServerDriver {
    listener: TcpListener,
}

impl ServerDriver {
    pub fn create_server(port: u16) -> Self {
        // start TCP
        info!("starting server at port: {}", port);
        let listener = TcpListener::bind(create_localhost_addr(port)).expect("cannot start TCP");
        ServerDriver { listener }
    }

    pub fn accept_conn(&self) {
        info!("waiting for new TCP connection....");
        let (str, addr) = self.listener.accept().unwrap();
        info!("accepted new client at: {}", addr);
        let mut server = TlsTcpServer::new(str);

        // state machine
        let mut sm = ServerStateMachine::new(server.create_tls_str());
        sm.start()
    }
}

pub fn client_send_files(paths: Vec<PathBuf>, addr: String) {
    let socket_addr: SocketAddr = addr
        .parse()
        .expect("Invalid server address, a valid example: 127.0.0.1:8080");
    info!("sending files: {:?} to {}", paths, socket_addr);

    // check all files are exists
    for p in &paths {
        let f = File::open(&p).unwrap();
        if !f.metadata().unwrap().is_file() {
            panic!("invalid file: {:?}", p);
        }
    }

    let mut client = TlsTcpClient::connect(socket_addr);
    let mut cm = ClientStateMachine::new(client.create_tls_str(), &paths);
    cm.start()
}

fn create_localhost_addr(port: u16) -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
}
