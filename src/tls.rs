use log::debug;
use std::{net::TcpStream, sync::Arc};

use rcgen::generate_simple_self_signed;
use rustls::{
    Certificate, ClientConnection, NoClientAuth, PrivateKey, ProtocolVersion, RootCertStore,
    ServerConfig, ServerConnection, Stream,
};
use std::net::SocketAddr;

pub struct TlsTcpServer {
    str: TcpStream,
    conn: ServerConnection,
}

impl TlsTcpServer {
    pub fn new(str: TcpStream) -> Self {
        // create key
        let keypair = KeyPair::new();

        // create config
        let mut config =
            ServerConfig::with_cipher_suites(NoClientAuth::new(), rustls::ALL_CIPHERSUITES);
        let cert = keypair.signed_public_key();
        let private_key = keypair.get_private_key();
        config
            .set_single_cert(vec![cert], private_key)
            .expect("bad certificates/private key");
        debug!(
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
}

pub struct TlsTcpClient {
    str: TcpStream,
    conn: ClientConnection,
}

impl TlsTcpClient {
    pub fn connect(addr: SocketAddr) -> Self {
        let str = TcpStream::connect(addr).unwrap();
        let root_store = RootCertStore::empty();
        let mut config = rustls::ClientConfig::new(root_store, &[], rustls::ALL_CIPHERSUITES);
        config
            .dangerous()
            .set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));
        debug!(
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

pub struct KeyPair {
    inner_cert: rcgen::Certificate,
}

impl KeyPair {
    pub fn new() -> Self {
        // generate
        let subject_alt_names = vec!["localhost".to_string()];
        let inner_cert = generate_simple_self_signed(subject_alt_names).unwrap();

        KeyPair { inner_cert }
    }

    pub fn get_private_key(&self) -> PrivateKey {
        PrivateKey(self.inner_cert.serialize_private_key_der())
    }

    pub fn signed_public_key(&self) -> Certificate {
        Certificate(self.inner_cert.serialize_der().unwrap())
    }
}

mod danger {
    pub struct NoCertificateVerification {}

    impl rustls::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &rustls::Certificate,
            _intermediates: &[rustls::Certificate],
            _dns_name: webpki::DnsNameRef<'_>,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp: &[u8],
            _now: std::time::SystemTime,
        ) -> Result<rustls::ServerCertVerified, rustls::Error> {
            Ok(rustls::ServerCertVerified::assertion())
        }
    }
}
