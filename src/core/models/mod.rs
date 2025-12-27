use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::core::enums::enums::FileType;

pub type EntryRc = Rc<RefCell<Entry>>;

#[derive(Clone, Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub ftype: FileType,
    pub is_open: bool,
    pub children: Vec<EntryRc>,
}
