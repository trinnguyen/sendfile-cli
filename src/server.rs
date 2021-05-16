use std::collections::HashMap;
use crate::{FileInfo, AcceptPacket};
use rustls::{Stream, ServerConnection};
use std::net::TcpStream;
use std::io::{Read, Write};

enum ServerState {
    Init,
    WaiForAnswer,
    Cancel,
    SendFile,
    DidInputStartFile,
    DidInputData,
    DidInputEndFile,
    DidInputFinish,
    Error
}

struct ServerStateMachine<'a> {
    state: ServerState,
    map_file_info: HashMap<usize, FileInfo>,
    tls: Stream<'a, ServerConnection, TcpStream>
}

impl <'a> ServerStateMachine<'a> {
    fn new(tls: Stream<'a, ServerConnection, TcpStream>) -> Self {
        ServerStateMachine {
            state: ServerState::Init,
            map_file_info: HashMap::new(),
            tls
        }
    }

    fn next(&mut self) {
        match self.state {
            ServerState::Init => {
                // expect input send
                let mut buf = [0; 1024];
                let len = self.tls.read(&mut buf).unwrap();

                // parse packet
                if let Some(packet) = SendPacket::parse(&buf[0..len]) {
                    packet.vec_file_info.iter().for_each(|f| { self.map_file_info.insert(f.size, f.clone()); } );
                    self.next_state(ServerState::WaiForAnswer)
                } else {
                    self.next_state(ServerState::Error)
                }
            }
            ServerState::WaiForAnswer => {
                // TODO ask for user input: accept or reject
                // default with accept

                // send success
                self.tls.write(&AcceptPacket::new().as_bytes());

                // next
                self.next_state(ServerState::SendFile)
            }
            ServerState::Cancel => {
                self.close()
            }
            ServerState::SendFile => {
                if let Some(packet) = StartFilePacket::parse() {
                    self.next_state()
                }
            }
            ServerState::DidInputStartFile => {}
            ServerState::DidInputData => {}
            ServerState::DidInputEndFile => {}
            ServerState::DidInputFinish => {}
            ServerState::Error => {
                self.close()
            }
        }
    }

    fn next_state(&mut self, state: ServerState) {
        self.state = state;
        self.next()
    }

    fn close(&mut self) {
        self.state = ServerState::Init;
        self.map_file_info.clear();
    }
}