use crate::packet::Packet;
pub use std::io::{BufReader, Read, Result, Write};

pub struct Streamer<S: Read + Write> {
    str: S,
}

impl<S: Read + Write> Streamer<S> {
    pub fn new(str: S) -> Self {
        Streamer { str }
    }

    /// convert to bytes array and write to socket
    /// [1 byte for action] + [2 bytes for len] + [additional data]
    pub fn write_packet(&mut self, packet: Packet) -> Result<usize> {
        let vec = Self::packet_to_bytes(packet);
        self.str.write(&vec).map(|l| {
            self.str.flush().unwrap();
            l
        })
    }

    /// read packet from socket
    /// [1 byte for action] + [2 bytes for len] + [additional data]
    pub fn read_packet(&mut self) -> Result<Packet> {
        // read action (1 byte)
        let action = match self.read_action() {
            Ok(act) => act,
            Err(e) => return Err(e),
        };

        // read len (2 bytes)
        let len = match self.read_len() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let data_buf = if len > 0 {
            let mut buf = vec![0_u8; len as usize];
            match self.str.read_exact(&mut buf) {
                Ok(_) => buf,
                Err(e) => return Err(e),
            }
        } else {
            vec![]
        };
        // println!("received packet: {}, data len: {}", action, len);
        Packet::from_data(action, &data_buf)
    }

    /// convert packet to bytes
    fn packet_to_bytes(packet: Packet) -> Vec<u8> {
        let action = packet.get_action();
        let data = packet.get_data();
        let len = data.len() as u16;

        let cap = 1 + 2 + len;
        let mut vec: Vec<u8> = Vec::with_capacity(cap as usize);
        vec.push(action);
        len.to_le_bytes().iter().for_each(|b| vec.push(*b));
        data.iter().for_each(|b| vec.push(*b));
        // println!("sent packet: {}, data len: {}", action, len);
        vec
    }

    fn read_action(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        match self.str.read_exact(&mut buf) {
            Ok(_) => Ok(*buf.first().unwrap()),
            Err(err) => Err(err),
        }
    }

    fn read_len(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.str
            .read_exact(&mut buf)
            .map(|_| u16::from_ne_bytes(buf))
    }
}
