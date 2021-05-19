use std::path::{PathBuf};
use std::fs::File;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u64
}

impl FileInfo {
    pub fn from_path(path: &PathBuf) -> Self {
        let f = File::open(path).unwrap();
        let meta = f.metadata().unwrap();
        FileInfo {
            name: String::from(path.file_name().map(|s| s.to_str()).unwrap().unwrap()),
            size: meta.len()
        }
    }
}