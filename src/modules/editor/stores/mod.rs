pub mod context;
pub mod editor;
pub mod file;
pub mod hotkeys;
pub mod theme;

pub use editor::editor_interactions::{EditorInteractionsStore, Tab, editor_interactions_store};
pub use file::file_actions::FileActionsStore;
pub use file::file_interactions::FileInteractionsStore;
pub use file::file_services::FileServicesStore;
pub use theme::{ThemeInteractionsStore, theme_store};

use crate::core::stores::icons::IconsInteractionsStore;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct EditorStores {
    pub icons: Rc<IconsInteractionsStore>,
    pub file_interactions: Rc<RefCell<FileInteractionsStore>>,
    pub file_actions: Rc<RefCell<FileActionsStore>>,
    pub theme: Rc<ThemeInteractionsStore>,
}

impl EditorStores {
    pub fn new(
        icons: Rc<IconsInteractionsStore>,
        file_interactions: Rc<RefCell<FileInteractionsStore>>,
        file_actions: Rc<RefCell<FileActionsStore>>,
        theme: Rc<ThemeInteractionsStore>,
    ) -> Self {
        Self {
            icons,
            file_interactions,
            file_actions,
            theme,
        }
    }
}
