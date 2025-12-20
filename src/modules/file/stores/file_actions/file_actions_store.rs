use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct FileActionsStore {
    pub opened_file: Rc<RefCell<Option<PathBuf>>>,
    pub opened_text: Rc<RefCell<String>>,
    pub is_dirty: Rc<RefCell<bool>>,
}

impl FileActionsStore {
    pub fn new() -> Self {
        Self {
            opened_file: Rc::new(RefCell::new(None)),
            opened_text: Rc::new(RefCell::new(String::new())),
            is_dirty: Rc::new(RefCell::new(false)),
        }
    }

    pub fn open_file(&mut self, path: &PathBuf) {
        if let Ok(text) = std::fs::read_to_string(path) {
            *self.opened_file.borrow_mut() = Some(path.clone());
            *self.opened_text.borrow_mut() = text;
            *self.is_dirty.borrow_mut() = false;
        }
    }

    pub fn save_file(&mut self) {
        if let Some(path) = self.opened_file.borrow().as_ref() {
            let text = self.opened_text.borrow().clone();
            if let Err(e) = std::fs::write(path, text) {
                eprintln!("Failed to save file: {}", e);
            } else {
                *self.is_dirty.borrow_mut() = false;
            }
        }
    }

    pub fn refresh_files(&mut self) {}

    pub fn create_new_file(&mut self) {}
}

impl Default for FileActionsStore {
    fn default() -> Self {
        Self::new()
    }
}
