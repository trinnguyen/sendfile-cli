#[derive(Debug,PartialEq, Eq)]
pub enum PacketAction {
    Send = 0,
    Accept = 1,
    Reject = 2,
    StartFile = 3,
    EndFile = 4,
    FileData = 5,
    Finish = 6
}

// parse action from u8
impl PacketAction {
    pub fn from_u8(val: u8) -> Option<Self> {
        let res = Some(match val {
            0 => PacketAction::Send,
            1 => PacketAction::Accept,
            2 => PacketAction::Reject,
            3 => PacketAction::StartFile,
            4 => PacketAction::EndFile,
            5 => PacketAction::FileData,
            6 => PacketAction::Finish,
            _ => return None
        });
        println!("parse action: {:?}", res);
        res
    }

    pub fn expect_action(buf: &[u8], expected_action: PacketAction) -> bool {
        match buf.first().map_or(None, |v| PacketAction::from_u8(*v)) {
            Some(action) if action == expected_action => true,
            _ => false
        }
    }

    pub fn as_u8(self) -> u8 {
        return self as u8;
    }
}

#[cfg(test)]
mod tests {
    use crate::packet::action::PacketAction;

    #[test]
    fn test_action_convert() {
        assert_eq!(PacketAction::Send, PacketAction::from_u8(0).unwrap());
        assert_eq!(PacketAction::Accept, PacketAction::from_u8(1).unwrap());
        assert_eq!(PacketAction::Reject, PacketAction::from_u8(2).unwrap());
        assert_eq!(PacketAction::StartFile, PacketAction::from_u8(3).unwrap());
        assert_eq!(PacketAction::EndFile, PacketAction::from_u8(4).unwrap());
        assert_eq!(PacketAction::FileData, PacketAction::from_u8(5).unwrap());
        assert_eq!(PacketAction::Finish, PacketAction::from_u8(6).unwrap());
        assert!(PacketAction::from_u8(7).is_none());
        assert!(PacketAction::from_u8(8).is_none());
    }
}