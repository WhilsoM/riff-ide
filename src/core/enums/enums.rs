use std::path::PathBuf;

#[derive(PartialEq, Clone, Debug)]
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
    SaveFile(PathBuf),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Hotkeys {
    ToggleExplorer, // cmd + b
    CloseFile,      // cmd + w
    FindFile,       // cmd + p
    FindSettings,   // cmd + shift + p
    FindText,       // cmd + f
}
