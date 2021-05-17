use crate::common::{PacketAsBytes, ParsePacket};

use super::{action::PacketAction, util::Util};

pub struct EndFilePacket {
}

impl EndFilePacket {
    pub fn new() -> Self {
        EndFilePacket {}
    }
}

impl ParsePacket for EndFilePacket {
    fn parse(buf: &[u8]) -> Option<Self> {
        if PacketAction::expect_action(buf, PacketAction::EndFile) {
            Some(EndFilePacket::new())
        } else {
            None
        }
    }
}

impl PacketAsBytes for EndFilePacket {
    fn as_bytes(&self) -> Vec<u8> {
        Util::to_bytes(PacketAction::EndFile)
    }
}