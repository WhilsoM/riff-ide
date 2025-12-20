use std::path::PathBuf;

use crate::core::enums::enums::FileType;

#[derive(Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub ftype: FileType,
    pub is_open: bool,
    pub children: Vec<Entry>,
}
