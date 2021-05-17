use crate::{common::{FileInfo, PacketAsBytes, ParsePacket}, packet::action::PacketAction};

use super::util::Util;


pub struct SendPacket {
    pub vec_file_info: Vec<FileInfo>
}

impl SendPacket {
    pub fn new(input: &Vec<FileInfo>) -> Self {
        return SendPacket { 
            vec_file_info: input.iter().map(|f| f.clone()).collect()
        };
    }
}

impl ParsePacket for SendPacket {
    fn parse(buf: &[u8]) -> Option<Self> {
        let opt_data: Option<Vec<FileInfo>> = Util::parse_data_with_json(buf, PacketAction::Send);
        opt_data.map(|dt| SendPacket{ vec_file_info: dt })
    }
}

impl PacketAsBytes for SendPacket {
    fn as_bytes(&self) -> Vec<u8> {
        Util::to_bytes_with_json_data(PacketAction::Send, &self.vec_file_info)
    }
}
