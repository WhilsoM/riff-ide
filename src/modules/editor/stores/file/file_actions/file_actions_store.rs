use std::path::PathBuf;

use eframe::egui;

use crate::store;

store! {
    pub struct FileActionsStore {
        opened_file: Option<PathBuf> = None,
        opened_text: String = String::new(),
        is_dirty: bool = false,
    }

    open_file(&self, ctx: &egui::Context, path: PathBuf) {
        if let Ok(text) = std::fs::read_to_string(&path) {
            let mut reactive = self.reactive(ctx);
            *reactive.opened_file() = Some(path);
            *reactive.opened_text() = text;
            *reactive.is_dirty() = false;
        }
    }

    save_file(&self, ctx: &egui::Context) {
        let path = self.opened_file.get(ctx);
        if let Some(path) = path.as_ref() {
            let text = self.opened_text.get(ctx);
            if let Err(e) = std::fs::write(path, text.as_str()) {
                eprintln!("Failed to save file: {}", e);
            } else {
                let mut reactive = self.reactive(ctx);
                *reactive.is_dirty() = false;
            }
        }
    }

    refresh_files(&self, _ctx: &egui::Context) {}

    create_new_file(&self, _ctx: &egui::Context) {}
}

pub fn file_actions_store() -> std::cell::Ref<'static, FileActionsStore> {
    FileActionsStore::instance()
}
