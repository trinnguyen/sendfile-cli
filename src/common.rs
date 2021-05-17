use std::{fs::File, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u64
}

impl FileInfo {
    pub fn new() -> Self {
        FileInfo {
            name: String::new(),
            size: 0
        }
    }

    pub fn from_path(path: &str) -> Self {
        let p = Path::new(path);
        let f = File::open(p).unwrap();
        let meta = f.metadata().unwrap();
        FileInfo {
            name: String::from(p.file_name().map(|s| s.to_str()).unwrap().unwrap()),
            size: meta.len()
        }
    }
}

pub trait ParsePacket: Sized {
    fn parse(buf: &[u8]) -> Option<Self>;
}

pub trait PacketAsBytes: Sized {
    fn as_bytes(&self) -> Vec<u8>;
}