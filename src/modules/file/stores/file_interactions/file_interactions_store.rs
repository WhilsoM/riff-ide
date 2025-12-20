use crate::core::enums::enums::UiAction;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct FileInteractionsStore {
    pub pending_action: Rc<RefCell<Option<UiAction>>>,
    pub selected_file: Rc<RefCell<Option<PathBuf>>>,
}

impl FileInteractionsStore {
    pub fn new() -> Self {
        Self {
            pending_action: Rc::new(RefCell::new(None)),
            selected_file: Rc::new(RefCell::new(None)),
        }
    }

    pub fn handle_file_click(&mut self, path: &PathBuf) {
        *self.pending_action.borrow_mut() = Some(UiAction::OpenFile(path.clone()));
        *self.selected_file.borrow_mut() = Some(path.clone());
    }

    pub fn handle_folder_click(&mut self, path: &PathBuf) {
        *self.selected_file.borrow_mut() = Some(path.clone());
    }

    pub fn take_action(&mut self) -> Option<UiAction> {
        self.pending_action.borrow_mut().take()
    }

    pub fn handle_save_file(&mut self) -> Option<UiAction> {}
}

impl Default for FileInteractionsStore {
    fn default() -> Self {
        Self::new()
    }
}
