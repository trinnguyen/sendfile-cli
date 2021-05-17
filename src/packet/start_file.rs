use core::str;
use std::usize;

use serde::{Deserialize, Serialize};
use crate::common::{FileInfo, PacketAsBytes, ParsePacket};

use super::{action::PacketAction, util::Util};

pub struct StartFilePacket {
    pub data: StartFileData
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartFileData {
    pub file_info: FileInfo,
    pub index: usize,
    pub total: usize
}

impl StartFilePacket {
    pub fn new(file_info: FileInfo, index: usize, total: usize) -> Self {
        StartFilePacket{ data: StartFileData {
            file_info,
            index,
            total
        }}
    }
}

impl ParsePacket for StartFilePacket {
    fn parse(buf: &[u8]) -> Option<Self> {
        let opt_data: Option<StartFileData> = Util::parse_data_with_json(buf, PacketAction::StartFile);
        opt_data.map(|dt| StartFilePacket{ data: dt })
    }
}

impl PacketAsBytes for StartFilePacket {
    fn as_bytes(&self) -> Vec<u8> {
        Util::to_bytes_with_json_data(PacketAction::StartFile, &self.data)
    }
}