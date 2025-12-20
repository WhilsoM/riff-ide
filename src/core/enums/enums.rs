use std::path::PathBuf;

#[derive(PartialEq, Clone)]
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

#[derive(Clone)]
pub enum UiAction {
    OpenFile(PathBuf),
}
