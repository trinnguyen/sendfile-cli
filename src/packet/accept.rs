use crate::common::{PacketAsBytes, ParsePacket};

use super::{action::PacketAction, util::Util};

pub struct AcceptPacket {
    
}

impl AcceptPacket {
    pub fn new() -> Self {
        AcceptPacket{}
    }
}



impl ParsePacket for AcceptPacket {
    fn parse(buf: &[u8]) -> Option<Self> {
        if PacketAction::expect_action(buf, PacketAction::Accept) {
            Some(AcceptPacket::new())
        } else {
            None
        }
    }
}

impl PacketAsBytes for AcceptPacket {
    fn as_bytes(&self) -> Vec<u8> {
        Util::to_bytes(PacketAction::Accept)
    }
}