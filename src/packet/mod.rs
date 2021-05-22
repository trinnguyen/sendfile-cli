pub mod file_info;
pub mod start_file;

use crate::packet::file_info::FileInfo;
use crate::packet::start_file::StartFileData;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result};

pub enum Packet {
    Send(Vec<FileInfo>),
    Accept,
    Reject,
    StartFile(StartFileData),
    FileData(Vec<u8>),
    EndFile,
    Finish,
}

impl Packet {
    pub fn from_data(action: u8, buf: &[u8]) -> Result<Self> {
        match action {
            0 => Self::parse_json::<Vec<FileInfo>>(buf).map(Packet::Send),
            1 => Ok(Packet::Accept),
            2 => Ok(Packet::Reject),
            3 => Self::parse_json::<StartFileData>(buf).map(Packet::StartFile),
            4 => Ok(Packet::FileData(buf.iter().copied().collect())),
            5 => Ok(Packet::EndFile),
            6 => Ok(Packet::Finish),
            _ => Err(Error::from(ErrorKind::InvalidData)),
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
            Packet::Finish => 6,
        }
    }

    pub fn get_data(self) -> Vec<u8> {
        match self {
            Packet::Send(data) => Self::json_bytes(data),
            Packet::StartFile(data) => Self::json_bytes(data),
            Packet::FileData(data) => data,
            _ => vec![],
        }
    }

    fn parse_json<'a, T>(buf: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        serde_json::from_slice::<T>(buf).map_err(std::io::Error::from)
    }

    fn json_bytes<T>(data: T) -> Vec<u8>
    where
        T: Serialize,
    {
        let json = serde_json::to_string(&data).unwrap();
        json.into_bytes()
    }
}
