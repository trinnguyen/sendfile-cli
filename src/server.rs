use rustls::{ServerConnection, Stream};
use std::{
    io::{Read, Write, Result},
    net::TcpStream,
};

use crate::{common::{FileInfo, PacketAsBytes, ParsePacket}, packet::{accept::AcceptPacket, end_file::EndFilePacket, file_data::FileDataPacket, finish::FinishPacket, reject::RejectPacket, send::SendPacket, start_file::StartFilePacket, util::Util}};

#[derive(Debug)]
enum ServerState {
    Init,
    InternalAnswer,
    WaitForFile,
    StartReceivingFile,
    ReceiveFileData,
    EndReceivingFile,
    Finish,
    Error,
}

pub struct ServerStateMachine<'a> {
    state: ServerState,
    tls: Stream<'a, ServerConnection, TcpStream>,
    vec_file_info: Vec<FileInfo>,
}

impl<'a> ServerStateMachine<'a> {
    pub fn new(tls: Stream<'a, ServerConnection, TcpStream>) -> Self {
        ServerStateMachine {
            state: ServerState::Init,
            tls,
            vec_file_info: Vec::new(),
        }
    }

    /// start the state machine
    pub fn start(&mut self) {
        self.state = ServerState::Init;
        self.next()
    }

    /// state machine for server (receiver)
    fn next(&mut self) {
        println!("process state: {:?}", self.state);
        match self.state {
            ServerState::Init => {
                // reset
                self.vec_file_info.clear();

                // parse packet
                match self.read_packet() {
                    Some(SendPacket { vec_file_info }) => {
                        vec_file_info
                            .iter()
                            .for_each(|f| self.vec_file_info.push(f.clone()));
                        self.next_state(ServerState::InternalAnswer)
                    }
                    None => self.error(),
                }
            }
            ServerState::InternalAnswer => {
                println!("internal answer for request: {:?}", self.vec_file_info);

                // send accept of cancel
                let is_accepted = true;
                if is_accepted {
                    match self.write_packet(&AcceptPacket::new()) {
                        Ok(_) => self.next_state(ServerState::WaitForFile),
                        Err(_) => self.error()
                    }
                } else {
                    match self.write_packet(&RejectPacket::new()) {
                        Ok(_) => self.next_state(ServerState::Finish),
                        Err(_) => self.error()
                    }
                };
            }
            ServerState::WaitForFile => {
                let opt: Option<StartFilePacket> = self.read_packet();
                match opt {
                    Some(start_file) => self.process_start_file(start_file),
                    None => self.error(),
                }
                self.next_state(ServerState::StartReceivingFile)
            }
            ServerState::StartReceivingFile => {
                let file_data: Option<FileDataPacket> = self.read_packet();
                match file_data {
                    Some(dt) => self.process_file_data(dt),
                    None => self.error(),
                }
            }
            ServerState::ReceiveFileData => {
                let mut buf = [0; 1024];
                let len = self.tls.read(&mut buf).unwrap();
                if let Some(file_data) = FileDataPacket::parse(&buf[0..len]) {
                    self.process_file_data(file_data)
                } else if let Some(end_file) = EndFilePacket::parse(&buf[0..len]) {
                    self.process_end_file(end_file)
                }
            }
            ServerState::EndReceivingFile => {
                let mut buf = [0; 1024];
                let len = self.tls.read(&mut buf).unwrap();
                if let Some(start_file) = StartFilePacket::parse(&buf[0..len]) {
                    self.process_start_file(start_file)
                } else if let Some(_) = FinishPacket::parse(&buf[0..len]) {
                    self.next_state(ServerState::Finish)
                }
            }
            ServerState::Finish => {
                // close connection
                self.tls.flush().expect("failed to close connection")
            }
            ServerState::Error => {
                //TODO print error
            }
        }
    }

    fn process_start_file(&mut self, packet: StartFilePacket) {
        println!("start receiving file: {:?}", packet.data);
        self.next_state(ServerState::StartReceivingFile)
    }

    fn process_file_data(&mut self, packet: FileDataPacket) {
        //TODO cache file data and continue
        self.next_state(ServerState::ReceiveFileData);
    }

    fn process_end_file(&mut self, packet: EndFilePacket) {
        //TODO start file
        self.next_state(ServerState::EndReceivingFile)
    }

    fn next_state(&mut self, state: ServerState) {
        self.state = state;
        self.next()
    }

    fn error(&mut self) {
        self.next_state(ServerState::Error)
    }

    fn close(&mut self) {
        self.state = ServerState::Init;
    }

    fn write_packet<T>(&mut self, packet: &T) -> Result<usize>
    where
        T: PacketAsBytes,
    {
        let res = self.tls.write(&packet.as_bytes());
        println!("wrote TCP packet: {:?}", res);
        res
    }

    fn read_packet<T>(&mut self) -> Option<T>
    where
        T: ParsePacket,
    {
        // expect input send
        let mut buf = [0; 1024];
        let len = self.tls.read(&mut buf).unwrap();
        T::parse(&buf[0..len])
    }
}
