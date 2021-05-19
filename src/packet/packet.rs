use crate::packet::start_file::StartFileData;
use std::io::{Result, Error, ErrorKind};
use serde::{Serialize, Deserialize};
use crate::packet::file_info::FileInfo;

pub enum Packet {
    Send(Vec<FileInfo>),
    Accept,
    Reject,
    StartFile(StartFileData),
    FileData(Vec<u8>),
    EndFile,
    Finish
}

impl Packet {

    pub fn from_data(action: u8, buf: &[u8]) -> Result<Self> {
        match action {
            0 => Self::parse_json::<Vec<FileInfo>>(buf).map(|dt| Packet::Send(dt)),
            1 => Ok(Packet::Accept),
            2 => Ok(Packet::Reject),
            3 => Self::parse_json::<StartFileData>(buf).map(|dt| Packet::StartFile(dt)),
            4 => Ok(Packet::FileData(buf.iter().map(|b| *b).collect())),
            5 => Ok(Packet::EndFile),
            6 => Ok(Packet::Finish),
            _ => Err(Error::from(ErrorKind::InvalidData))
        }
    }

    pub fn get_action(&self) -> u8 {
        match self {
            Packet::Send(_) => 0,
            Packet::Accept => 1,
            Packet::Reject => 2,
            Packet::StartFile(_) => 3,
            Packet::FileData(_) => 4,
            Packet::EndFile => 5,
            Packet::Finish => 6
        }
    }

    pub fn get_data<'a>(self) -> Vec<u8> {
        match self {
            Packet::Send(data) => Self::json_bytes(data),
            Packet::StartFile(data) => Self::json_bytes(data),
            Packet::FileData(data) => data,
            _ => vec!()
        }
    }

    fn parse_json<'a, T>(buf: &'a [u8]) -> Result<T> where T: Deserialize<'a> {
        serde_json::from_slice::<T>(buf).map_err(|e| std::io::Error::from(e))
    }

    fn json_bytes<'a, T>(data: T) -> Vec<u8> where T: Serialize {
        let json = serde_json::to_string(&data).unwrap();
        json.into_bytes()
    }
}