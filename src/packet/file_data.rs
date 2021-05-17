use crate::common::{PacketAsBytes, ParsePacket};

use super::action::PacketAction;

pub struct FileDataPacket {
    data: Vec<u8>
}

impl FileDataPacket {
    pub fn new() -> Self {
        return FileDataPacket { data: vec![] };
    }
}

impl ParsePacket for FileDataPacket {
    fn parse(buf: &[u8]) -> Option<Self> {
        if PacketAction::expect_action(buf, PacketAction::FileData) {
            Some(FileDataPacket::new())
        } else {
            None
        }
    }
}

impl PacketAsBytes for FileDataPacket {
    fn as_bytes(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(PacketAction::FileData.as_u8());
        self.data.iter().for_each(|b| vec.push(*b));
        vec
    }
}