use crate::core::enums::enums::UiAction;
use crate::modules::editor::stores::file::file_actions::file_actions_store;
use crate::modules::editor::stores::{Tab, editor_interactions_store};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use eframe::egui;

use crate::store;

store! {
    pub struct FileInteractionsStore {
        pending_action:Option<UiAction> = None,
        selected_file:Option<PathBuf> = None,
    }

    handle_file_click(&mut self, ctx: &egui::Context, path: &PathBuf) {
      let mut reactive = self.reactive(ctx);
      *reactive.pending_action() = Some(UiAction::OpenFile(path.clone()));

      let file_actions_store = file_actions_store();
      file_actions_store.open_file(ctx, path.clone());

      // TODO: показывать что такие файлы нельзя открыть "test_rsx" (без расширения)
      if let Some(file) = path.file_name() {
          if !file.to_string_lossy().contains(".") {
              return;
          }
      }

      *reactive.selected_file() = Some(path.clone());

        println!(
            "FILE ACTIONS STORE PATH: {}, handle_file_click path: {}",
            file_actions_store
                .opened_file
                .clone()
                .get(&eframe::egui::Context::default())
                .as_ref()
                .map_or("None".to_string(), |p| p.to_string_lossy().to_string()),
            path.to_string_lossy(),
        );

        let editor = editor_interactions_store();

        for tab in editor.tabs.borrow().iter() {
            if &tab.path == path {
                return;
            }
        }

        let tab = Tab {
            path: path.clone(),
            content: Rc::new(RefCell::new(
                file_actions_store.opened_text.borrow().to_string(),
            )),
            original_content: String::new(),
            is_dirty: false,
        };

        editor.tabs.borrow_mut().push(tab)
    }

    handle_folder_click(&mut self, path: &PathBuf) {
        *self.selected_file.borrow_mut() = Some(path.clone());
    }

    take_action(&mut self) -> Option<UiAction> {
        self.pending_action.borrow_mut().take()
    }

    handle_save_file(&mut self) -> Option<UiAction> {
        None
    }
}

pub fn file_interactions_store() -> std::cell::Ref<'static, FileInteractionsStore> {
    FileInteractionsStore::instance()
}

// pub struct FileInteractionsStore {
//     pub pending_action: Rc<RefCell<Option<UiAction>>>,
//     pub selected_file: Rc<RefCell<Option<PathBuf>>>,
// }

// impl FileInteractionsStore {
//     new() -> Self {
//         Self {
//             pending_action: Rc::new(RefCell::new(None)),
//             selected_file: Rc::new(RefCell::new(None)),
//         }
//     }

//     pub fn handle_file_click(&mut self, path: &PathBuf) {
//         *self.pending_action.borrow_mut() = Some(UiAction::OpenFile(path.clone()));
//         *self.selected_file.borrow_mut() = Some(path.clone());

//         let file_actions_store = file_actions_store();
//         println!(
//             "FILE ACTIONS STORE PATH: {}, handle_file_click path: {}",
//             file_actions_store
//                 .opened_file
//                 .clone()
//                 .get(&eframe::egui::Context::default())
//                 .as_ref()
//                 .map_or("None".to_string(), |p| p.to_string_lossy().to_string()),
//             path.to_string_lossy(),
//         );

//         let editor = editor_interactions_store();

//         for tab in editor.tabs.borrow().iter() {
//             if &tab.path == path {
//                 return;
//             }
//         }

//         let tab = Tab {
//             path: path.clone(),
//             content: Rc::new(RefCell::new(
//                 file_actions_store.opened_text.borrow().to_string(),
//             )),
//             original_content: String::new(),
//             is_dirty: false,
//         };

//         editor.tabs.borrow_mut().push(tab)
//     }

//     pub fn handle_folder_click(&mut self, path: &PathBuf) {
//         *self.selected_file.borrow_mut() = Some(path.clone());
//     }

//     pub fn take_action(&mut self) -> Option<UiAction> {
//         self.pending_action.borrow_mut().take()
//     }

//     pub fn handle_save_file(&mut self) -> Option<UiAction> {
//         None
//     }
// }

// impl Default for FileInteractionsStore {
//     fn default() -> Self {
//         Self::new()
//     }
// }

thread_local! {
    static STORE: RefCell<Option<Rc<RefCell<FileInteractionsStore>>>> = RefCell::new(None);
}

pub fn init_store(store: Rc<RefCell<FileInteractionsStore>>) {
    STORE.with(|s| *s.borrow_mut() = Some(store));
}

pub fn get_store() -> Rc<RefCell<FileInteractionsStore>> {
    STORE.with(|s| {
        s.borrow()
            .as_ref()
            .expect("FileInteractionsStore not initialized")
            .clone()
    })
}

// store! {
//     pub struct FileActionsStore {
//         opened_file: Option<PathBuf> = None,
//         opened_text: String = String::new(),
//         is_dirty: bool = false,
//     }

//     open_file(&self, ctx: &egui::Context, path: PathBuf) {
//         if let Ok(text) = std::fs::read_to_string(&path) {
//             let mut reactive = self.reactive(ctx);
//             *reactive.opened_file() = Some(path);
//             *reactive.opened_text() = text;
//             *reactive.is_dirty() = false;
//         }
//     }

//     save_file(&self, ctx: &egui::Context) {
//         let path = self.opened_file.get(ctx);
//         if let Some(path) = path.as_ref() {
//             let text = self.opened_text.get(ctx);
//             if let Err(e) = std::fs::write(path, text.as_str()) {
//                 eprintln!("Failed to save file: {}", e);
//             } else {
//                 let mut reactive = self.reactive(ctx);
//                 *reactive.is_dirty() = false;
//             }
//         }
//     }

//     refresh_files(&self, _ctx: &egui::Context) {}

//     create_new_file(&self, _ctx: &egui::Context) {}
// }

// pub fn file_actions_store() -> std::cell::Ref<'static, FileActionsStore> {
//     FileActionsStore::instance()
// }
