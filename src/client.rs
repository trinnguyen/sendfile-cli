use crate::packet::file_info::FileInfo;
use crate::packet::start_file::StartFileData;
use crate::packet::Packet;
use crate::streamer::Streamer;
use std::path::PathBuf;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    usize,
};

enum ClientState {
    Init, // ask for sending files
    WaitForResponse,
    Accepted,
    StartSendingFile,
    SendFileData,
    EndSendingFile,
    Finish,
    Error,
}

pub struct ClientStateMachine<S>
where
    S: Read + Write,
{
    state: ClientState,
    str: Streamer<S>,
    items: Vec<PathBuf>,
    opt_reader: Option<BufReader<File>>,
    sent_size: usize,
    cur_index: usize,
}

impl<S> ClientStateMachine<S>
where
    S: Read + Write,
{
    pub fn new(s: S, items: &[PathBuf]) -> Self {
        ClientStateMachine {
            state: ClientState::Init,
            str: Streamer::new(s),
            items: items.to_vec(),
            opt_reader: None,
            sent_size: 0,
            cur_index: 0,
        }
    }

    /// start the state machine
    pub fn start(&mut self) {
        self.state = ClientState::Init;
        self.next()
    }

    /// state machine
    fn next(&mut self) {
        loop {
            match self.state {
                ClientState::Init => {
                    let infos: Vec<FileInfo> =
                        self.items.iter().map(|p| FileInfo::from_path(p)).collect();
                    match self.str.write_packet(Packet::Send(infos)) {
                        Ok(_) => self.state = ClientState::WaitForResponse,
                        _ => self.error(),
                    }
                }
                ClientState::WaitForResponse => match self.str.read_packet() {
                    Ok(Packet::Accept) => self.state = ClientState::Accepted,
                    Ok(Packet::Reject) => self.state = ClientState::Finish,
                    _ => self.error(),
                },
                ClientState::Accepted => self.process_start_file(),
                ClientState::StartSendingFile => {
                    self.process_file_data();
                }
                ClientState::SendFileData => {
                    self.process_file_data();
                }
                ClientState::EndSendingFile => {
                    let fin = self.cur_index >= self.total() - 1;
                    if !fin {
                        // increase current and continue
                        self.cur_index += 1;
                        self.process_start_file()
                    } else {
                        // finish
                        match self.str.write_packet(Packet::Finish) {
                            Ok(_) => self.state = ClientState::Finish,
                            Err(_) => self.error(),
                        }
                    }
                }
                ClientState::Finish => break,
                ClientState::Error => break,
            }
        }

        self.close()
    }

    fn total(&self) -> usize {
        self.items.len()
    }

    fn process_start_file(&mut self) {
        match self.items.get(self.cur_index) {
            Some(item) => {
                // read file
                match File::open(item) {
                    Ok(file) => {
                        self.opt_reader = Some(BufReader::with_capacity(61 * 1024, file));
                        self.sent_size = 0;
                    }
                    Err(_) => {
                        self.error();
                        return;
                    }
                }

                // send packet to server
                let data =
                    StartFileData::new(FileInfo::from_path(item), self.cur_index, self.items.len());
                match self.str.write_packet(Packet::StartFile(data)) {
                    Ok(_) => self.state = ClientState::StartSendingFile,
                    Err(_) => self.error(),
                }
            }
            None => self.error(),
        }
    }

    fn process_file_data(&mut self) {
        if let Some(reader) = self.opt_reader.as_mut() {
            let buf = reader.fill_buf().unwrap();
            let len = buf.len();
            if len > 0 {
                let vec: Vec<u8> = buf.iter().copied().collect();
                reader.consume(len);
                self.sent_size += len;
                match self.str.write_packet(Packet::FileData(vec)) {
                    Ok(_) => self.state = ClientState::SendFileData,
                    Err(_) => self.error(),
                };
            } else {
                self.process_end_file();
            }
        }
    }

    fn process_end_file(&mut self) {
        match self.str.write_packet(Packet::EndFile) {
            Ok(_) => self.state = ClientState::EndSendingFile,
            Err(_) => self.error(),
        }
    }

    fn error(&mut self) {
        self.state = ClientState::Error
    }

    fn close(&mut self) {
        //TODO shutdown
    }
}
