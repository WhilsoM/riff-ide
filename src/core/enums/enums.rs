use std::path::PathBuf;

#[derive(PartialEq)]
pub enum FileType {
    Folder,
    File,
    Symlink,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Icon {
    Folder,
    OpenFolder,
    File,
    Rust,
}

pub enum UiAction {
    OpenFile(PathBuf),
}
