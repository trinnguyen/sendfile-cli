use core::str;
use std::usize;

use serde::{Deserialize, Serialize};
use crate::packet::file_info::FileInfo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartFileData {
    pub file_info: FileInfo,
    pub index: usize,
    pub total: usize
}

impl StartFileData {
    pub fn new(file_info: FileInfo, index: usize, total: usize) -> Self {
        StartFileData {
            file_info,
            index,
            total
        }
    }
}