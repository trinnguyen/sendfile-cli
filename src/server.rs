use crate::packet::file_info::FileInfo;
use crate::packet::start_file::StartFileData;
use crate::packet::Packet;
use crate::streamer::Streamer;
use log::debug;
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Write},
    path::Path,
};

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

pub struct ServerStateMachine<S>
where
    S: Read + Write,
{
    state: ServerState,
    str: Streamer<S>,
    files: Vec<FileInfo>,
    opt_writer: Option<BufWriter<File>>,
}

impl<S> ServerStateMachine<S>
where
    S: Read + Write,
{
    pub fn new(s: S) -> Self {
        ServerStateMachine {
            state: ServerState::Init,
            str: Streamer::new(s),
            files: Vec::new(),
            opt_writer: None,
        }
    }

    /// start the state machine
    pub fn start(&mut self) {
        self.state = ServerState::Init;
        self.next()
    }

    /// state machine for server (receiver)
    fn next(&mut self) {
        debug!("process state: {:?}", self.state);
        loop {
            match self.state {
                ServerState::Init => {
                    // reset
                    self.files.clear();

                    // parse packet
                    match self.str.read_packet() {
                        Ok(Packet::Send(data)) => {
                            data.iter().for_each(|f| self.files.push(f.clone()));
                            self.state = ServerState::InternalAnswer;
                        }
                        _ => self.error(),
                    }
                }
                ServerState::InternalAnswer => {
                    debug!("internal answer for request: {:?}", self.files);

                    // send accept of cancel
                    let is_accepted = true;
                    if is_accepted {
                        match self.str.write_packet(Packet::Accept) {
                            Ok(_) => self.state = ServerState::WaitForFile,
                            Err(_) => self.error(),
                        }
                    } else {
                        match self.str.write_packet(Packet::Reject) {
                            Ok(_) => self.state = ServerState::Finish,
                            Err(_) => self.error(),
                        }
                    };
                }
                ServerState::WaitForFile => match self.str.read_packet() {
                    Ok(Packet::StartFile(data)) => self.process_start_file(data),
                    _ => self.error(),
                },
                ServerState::StartReceivingFile => match self.str.read_packet() {
                    Ok(Packet::FileData(data)) => self.process_file_data(data),
                    _ => self.error(),
                },
                ServerState::ReceiveFileData => match self.str.read_packet() {
                    Ok(Packet::EndFile) => self.process_end_file(),
                    Ok(Packet::FileData(data)) => self.process_file_data(data),
                    _ => self.error(),
                },
                ServerState::EndReceivingFile => match self.str.read_packet() {
                    Ok(Packet::StartFile(data)) => self.process_start_file(data),
                    Ok(Packet::Finish) => self.state = ServerState::Finish,
                    _ => self.error(),
                },
                ServerState::Finish => break,
                ServerState::Error => break,
            }
        }

        self.close()
    }

    fn process_start_file(&mut self, data: StartFileData) {
        debug!("start receiving file: {:?}", data);
        let path = Path::new("out").join(data.file_info.name);
        let opts = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();

        self.opt_writer = Some(BufWriter::new(opts));
        self.state = ServerState::StartReceivingFile
    }

    fn process_file_data(&mut self, data: Vec<u8>) {
        if let Some(writer) = self.opt_writer.as_mut() {
            match writer.write(&data) {
                Ok(_) => self.state = ServerState::ReceiveFileData,
                Err(_) => self.error(),
            }
        } else {
            self.error()
        }
    }

    fn process_end_file(&mut self) {
        if let Some(writer) = self.opt_writer.as_mut() {
            writer.flush().unwrap();
            self.state = ServerState::EndReceivingFile
        }
    }

    fn error(&mut self) {
        self.state = ServerState::Error
    }

    fn close(&mut self) {
        //TODO shutdown
    }
}
