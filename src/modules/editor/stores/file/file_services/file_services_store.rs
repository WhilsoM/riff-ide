use crate::core::{types::types::EntryRc, utils::utils::read_current_folder};
use std::path::PathBuf;

pub struct FileServicesStore;

impl FileServicesStore {
    pub fn new() -> Self {
        Self
    }

    pub fn read_directory(&self, path: &PathBuf) -> Vec<EntryRc> {
        read_current_folder(path)
    }

    pub fn read_file(&self, path: &PathBuf) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }

    pub fn write_file(&self, path: &PathBuf, content: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, content)
    }

    pub fn create_file(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::File::create(path)?;
        Ok(())
    }

    pub fn delete_file(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        std::fs::remove_file(path)
    }
}

impl Default for FileServicesStore {
    fn default() -> Self {
        Self::new()
    }
}
