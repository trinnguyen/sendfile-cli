use crate::{common::{PacketAsBytes, ParsePacket}, packet::{action::PacketAction, util::Util}};

pub struct RejectPacket {

}

impl RejectPacket {
    pub fn new() -> Self {
        RejectPacket{}
    }
}

impl ParsePacket for RejectPacket {
    fn parse(buf: &[u8]) -> Option<Self> {
        if PacketAction::expect_action(buf, PacketAction::Reject) {
            Some(RejectPacket::new())
        } else {
            None
        }
    }
}

impl PacketAsBytes for RejectPacket {
    fn as_bytes(&self) -> Vec<u8> {
        Util::to_bytes(PacketAction::Reject)
    }
}