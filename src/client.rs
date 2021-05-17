use std::{io::{Result, Read, Write}, net::TcpStream, usize};

use rustls::{ClientConnection, Stream};

use crate::{common::{FileInfo, PacketAsBytes, ParsePacket}, packet::{accept::AcceptPacket, end_file::EndFilePacket, file_data::FileDataPacket, finish::FinishPacket, reject::RejectPacket, send::SendPacket, start_file::StartFilePacket, util::Util}};

enum ClientState {
    Init, // ask for sending files
    WaitForResponse,
    Accepted,
    StartSendingFile,
    SendFileData,
    EndSendingFile,
    Finish,
    Error
}

pub struct ClientStateMachine<'a> {
    state: ClientState,
    tls: Stream<'a, ClientConnection, TcpStream>,
    vec_file_info: Vec<FileInfo>,
    current_file: usize
}

impl <'a> ClientStateMachine<'a> {
    pub fn new(tls: Stream<'a, ClientConnection, TcpStream>, vec_file_info: Vec<FileInfo>) -> Self {
        ClientStateMachine {
            state: ClientState::Init,
            tls,
            vec_file_info,
            current_file: 0
        }
    }

    /// start the state machine
    pub fn start(&mut self) {
        self.state = ClientState::Init;
        self.next()
    }

    /// state machine
    fn next(&mut self) {
        match self.state {
            ClientState::Init => {
                match self.write_packet(&SendPacket::new(&self.vec_file_info)) {
                    Ok(_) => self.next_state(ClientState::WaitForResponse),
                    _ => self.error()
                }
            }
            ClientState::WaitForResponse => {
                let mut buf = [0; 1024];
                let len = self.tls.read(&mut buf).unwrap();
                if let Some(_) = AcceptPacket::parse(&buf[0..len]) {
                    self.next_state(ClientState::Accepted)
                } else if let Some(_) = RejectPacket::parse(&buf[0..len]) {
                    self.next_state(ClientState::Finish)
                }
            }
            ClientState::Accepted => {
                self.process_start_file()
            }
            ClientState::StartSendingFile => {
                self.process_file_data();
            }
            ClientState::SendFileData => {
                let eof = false;
                if !eof {
                    self.process_file_data();
                } else {
                    self.process_end_file();
                }
            }
            ClientState::EndSendingFile => {
                let fin = self.current_file >= self.total() - 1;
                if !fin {
                    // increase current and continue
                    self.current_file += 1;
                    self.process_start_file()
                } else {
                    // finish
                    let finish = FinishPacket::new();
                    match self.write_packet(&finish) {
                        Ok(_) => self.next_state(ClientState::Finish),
                        Err(_) => self.error()
                    }
                }
            }
            ClientState::Finish => {
                self.close()
            }
            ClientState::Error => {
                self.close()
            }
        }
    }

    fn total(&self) -> usize {
        return self.vec_file_info.len();
    }

    fn process_start_file(&mut self) {
        match self.vec_file_info.get(self.current_file) {
            Some(info) => {
                let start_file = StartFilePacket::new(
                    info.clone(),
                    self.current_file,
                    self.vec_file_info.len()
                );
                match self.write_packet(&start_file) {
                    Ok(_) => self.next_state(ClientState::StartSendingFile),
                    Err(_) => self.error()
                }
            }
            None => self.error()
        }
    }

    fn process_file_data(&mut self) {
        let file_data = FileDataPacket::new();
        match self.write_packet(&file_data) {
            Ok(_) => self.next_state(ClientState::SendFileData),
            Err(_) => self.error()
        }
    }
    
    fn process_end_file(&mut self) {
        let end_file = EndFilePacket::new();
        match self.write_packet(&end_file) {
            Ok(_) => self.next_state(ClientState::EndSendingFile),
            Err(_) => self.error()
        }
    }
    
    fn next_state(&mut self, state: ClientState) {
        self.state = state;
        self.next()
    }

    fn error(&mut self) {
        self.next_state(ClientState::Error)
    }

    fn close(&mut self) {
        self.tls.flush().expect("failed to close connection")
    }

    fn write_packet<T>(&mut self, packet: &T) -> Result<usize> where T: PacketAsBytes {
        let res = self.tls.write(&packet.as_bytes());
        println!("wrote TCP packet: {:?}", res);
        res
    }
}