use std::{
    fs,
    io::BufReader,
    net::{TcpStream},
    sync::Arc,
};

use rustls::{
    ClientConnection, NoClientAuth, ProtocolVersion, RootCertStore, ServerConfig, ServerConnection,
    Stream,
};
use std::net::SocketAddr;

pub struct TlsTcpServer {
    str: TcpStream,
    conn: ServerConnection,
}

impl TlsTcpServer {

    pub fn new(str: TcpStream) -> Self {
        // create config
        let mut config =
            ServerConfig::with_cipher_suites(NoClientAuth::new(), rustls::ALL_CIPHERSUITES);
        let certs = TlsTcpServer::load_certs("certs/server-cert.pem");
        let privkey = TlsTcpServer::load_private_key("certs/server-key.pem");
        config
            .set_single_cert(certs, privkey)
            .expect("bad certificates/private key");
        println!(
            "TLSv1_3: {}",
            config.supports_version(ProtocolVersion::TLSv1_3)
        );
        let arc_config = Arc::new(config);
        let conn = ServerConnection::new(&arc_config);
        Self { str, conn }
    }

    pub fn create_tls_str(&mut self) -> Stream<ServerConnection, TcpStream> {
        Stream::new(&mut self.conn, &mut self.str)
    }

    fn load_private_key(filename: &str) -> rustls::PrivateKey {
        let keyfile = fs::File::open(filename).expect("cannot open private key file");
        let mut reader = BufReader::new(keyfile);
        loop {
            match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file")
            {
                Some(rustls_pemfile::Item::RSAKey(key)) => return rustls::PrivateKey(key),
                Some(rustls_pemfile::Item::PKCS8Key(key)) => return rustls::PrivateKey(key),
                None => break,
                _ => {}
            }
        }

        panic!(
            "no keys found in {:?} (encrypted keys not supported)",
            filename
        );
    }

    fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
        let certfile = fs::File::open(filename).expect("cannot open certificate file");
        let mut reader = BufReader::new(certfile);
        rustls_pemfile::certs(&mut reader)
            .unwrap()
            .iter()
            .map(|v| rustls::Certificate(v.clone()))
            .collect()
    }
}

pub struct TlsTcpClient {
    str: TcpStream,
    conn: ClientConnection,
}

impl TlsTcpClient {
    pub fn connect(addr: SocketAddr) -> Self {
        let str = TcpStream::connect(addr).unwrap();
        let certfile = fs::File::open("certs/ca-cert.pem").expect("Cannot open CA file");
        let mut reader = BufReader::new(certfile);
        let pemfile = rustls_pemfile::certs(&mut reader).unwrap();

        let mut root_store = RootCertStore::empty();
        root_store.add_parsable_certificates(&pemfile);
        let config = rustls::ClientConfig::new(root_store, &[], rustls::ALL_CIPHERSUITES);
        println!(
            "TLSv1_3: {}",
            config.supports_version(ProtocolVersion::TLSv1_3)
        );
        let arc_config = Arc::new(config);

        let dns_name = webpki::DnsNameRef::try_from_ascii_str("localhost").unwrap();
        let conn = ClientConnection::new(&arc_config, dns_name).unwrap();
        Self { conn, str }
    }

    pub fn create_tls_str(&mut self) -> Stream<ClientConnection, TcpStream> {
        Stream::new(&mut self.conn, &mut self.str)
    }
}
