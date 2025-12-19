use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use eframe::egui;

use crate::core::enums::enums::UiAction;
use crate::core::icons_store::IconStore;
use crate::core::models::Entry;
use crate::core::stores::app_name_store::AppNameStore;
use crate::core::ui::code_editor::code_editor;
use crate::core::ui::side_panel::side_panel;
use crate::core::utils::utils::read_current_folder;
use crate::store;

#[derive(Debug, Clone)]
pub struct Counter {
    pub counter: usize,
}

store! {
    #[derive(Debug)]
    pub struct ActionsStore {
        items: Vec<Counter> = vec![Counter{counter:1}, Counter{counter:2}, Counter{counter:3}],
        counter: u32 = 0,
    }

    increment(&self, ctx: &egui::Context) {
        let mut reactive = self.reactive(ctx);
        *reactive.counter() += 1;
    }

    update_item(&self, ctx: &egui::Context, i: usize) {
        let mut reactive = self.reactive(ctx);
        if let Some(elem) = reactive.items().get_mut(i) {
            elem.counter += 1;
        }
    }
}

pub struct MyApp {
    current_dir: PathBuf,
    files: Vec<Entry>,
    icons: IconStore,
    opened_file: Option<PathBuf>,
    opened_text: String,
    actions_store: Rc<RefCell<ActionsStore>>,
    app_name_store: AppNameStore,
}

impl MyApp {
    pub fn new(
        icons: IconStore,
        opened_file: Option<PathBuf>,
        opened_text: String,
        app_name_store: AppNameStore,
    ) -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let files = read_current_folder(&current_dir);
        let actions_store = Rc::new(RefCell::new(ActionsStore::new()));

        Self {
            current_dir,
            files,
            icons,
            opened_file,
            opened_text,
            actions_store,
            app_name_store,
        }
    }

    pub fn open_file(&mut self, path: &Path) {
        if let Ok(text) = fs::read_to_string(path) {
            self.opened_file = Some(path.to_path_buf());
            self.opened_text = text
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(UiAction::OpenFile(path)) = side_panel(&mut self.files, &self.icons, ctx) {
            self.open_file(&path);
        }

        code_editor(self.opened_file.as_ref(), &mut self.opened_text, ctx);
    }
}
