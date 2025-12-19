use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use eframe::egui;

use crate::core::enums::enums::{Icon, UiAction};
use crate::core::models::Entry;
use crate::core::ui::draw_entry::draw_entry;
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
}

impl MyApp {
    pub fn new(icons: IconStore, opened_file: Option<PathBuf>, opened_text: String) -> Self {
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
        let store = self.actions_store.clone();

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Панель файлов");
                let indent = 2;

                let mut pending_action: Option<UiAction> = None;

                for file in &mut self.files {
                    if let Some(action) = draw_entry(ui, file, &self.icons, indent) {
                        pending_action = Some(action);
                    }
                }
                if let Some(action) = pending_action {
                    match action {
                        UiAction::OpenFile(path) => {
                            self.open_file(&path);
                        }
                    }
                }

                if ui.button("Update").clicked() {
                    self.files = read_current_folder(&self.current_dir);
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(path) = &self.opened_file {
                ui.heading(path.file_name().unwrap().to_string_lossy());
                ui.separator();

                let items = store.borrow().items.get(ctx);
                for (i, elem) in items.iter().enumerate() {
                    let text = format!("{:?}", elem.counter);

                    if ui.button(text).clicked() {
                        store.borrow().update_item(ctx, i);
                    }
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.opened_text)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY),
                    );
                });
            } else {
                ui.label("Файл не открыт");
            }
        });
    }
}

pub struct IconStore {
    icons: HashMap<Icon, egui::TextureHandle>,
}

impl IconStore {
    pub fn new(ctx: &egui::Context) -> Self {
        let mut icons = HashMap::new();

        icons.insert(
            Icon::Folder,
            load_icon(
                ctx,
                "folder",
                include_bytes!("../../assets/icons/folder.png"),
            ),
        );

        icons.insert(
            Icon::OpenFolder,
            load_icon(
                ctx,
                "open_folder",
                include_bytes!("../../assets/icons/folder-open.png"),
            ),
        );

        icons.insert(
            Icon::File,
            load_icon(
                ctx,
                "file",
                include_bytes!("../../assets/icons/file-text.png"),
            ),
        );

        icons.insert(
            Icon::Rust,
            load_icon(
                ctx,
                "rust",
                include_bytes!("../../assets/icons/rust-icon.png"),
            ),
        );

        Self { icons }
    }

    pub fn get(&self, icon: &Icon) -> &egui::TextureHandle {
        &self.icons[icon]
    }
}

fn load_icon(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
    let image = image::load_from_memory(bytes).unwrap().to_rgba8();

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    ctx.load_texture(name, color_image, Default::default())
}
