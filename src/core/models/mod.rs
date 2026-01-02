use std::path::PathBuf;

use crate::core::{enums::enums::FileType, types::types::EntryRc};

#[derive(Clone, Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub ftype: FileType,
    pub is_open: bool,
    pub children: Vec<EntryRc>,
}
