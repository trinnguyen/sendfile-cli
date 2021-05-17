use crate::{common::{PacketAsBytes, ParsePacket}, packet::action::PacketAction};

use super::util::Util;

pub struct FinishPacket {
    
}

impl FinishPacket {
    pub fn new() -> Self {
        FinishPacket{}
    }
}



impl ParsePacket for FinishPacket {
    fn parse(buf: &[u8]) -> Option<Self> {
        if PacketAction::expect_action(buf, PacketAction::Finish) {
            Some(FinishPacket::new())
        } else {
            None
        }
    }
}

impl PacketAsBytes for FinishPacket {
    fn as_bytes(&self) -> Vec<u8> {
        Util::to_bytes(PacketAction::Finish)
    }
}