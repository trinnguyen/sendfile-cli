use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::common::PacketAsBytes;

use super::action::PacketAction;

pub struct Util {
}

impl Util {
    pub fn to_bytes(action: PacketAction) -> Vec<u8> {
        return vec!(action.as_u8());
    }

    pub fn to_bytes_with_json_data<T>(action: PacketAction, data: &T) -> Vec<u8> where T: ?Sized + Serialize,{
        let mut vec: Vec<u8> = Vec::new();
        vec.push(action.as_u8());
        
        // vec_file_info to json
        let json = serde_json::to_string(data).unwrap();
        println!("json: {}", json);
        let bytes = json.as_bytes();
        bytes.iter().for_each(|b| vec.push(*b));
        return vec;        
    }

    pub fn parse_data_with_json<'a, T>(buf: &'a [u8], expected_action: PacketAction) -> Option<T> where T: Deserialize<'a> {
        if PacketAction::expect_action(buf, expected_action) {
            let data: T = serde_json::from_slice(&buf[1..]).unwrap();
            Some(data)
        } else {
            None
        }
    }
}