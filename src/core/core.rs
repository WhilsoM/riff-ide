use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use eframe::egui;

use crate::core::enums::enums::{Icon, UiAction};
use crate::core::models::Entry;
use crate::core::ui::draw_entry::draw_entry;
use crate::core::utils::utils::read_current_folder;
use crate::store;

store! {
    pub struct ActionsStore {
        items: Vec<String> = Vec::new(),
        counter: u32 = 0,
    }

    get_all_items(&self) {
        // self здесь — аргумент метода
        let new_items = vec!["Item 1".to_string(), "Item 2".to_string()];
        self.items.set(new_items);
    }

    increment(&self, ctx: &egui::Context) {
        let val = self.counter.get(ctx);
        self.counter.set(val + 1);
    }
}

// Структура приложения
pub struct MyApp {
    current_dir: PathBuf,
    files: Vec<Entry>,
    icons: IconStore,
    opened_file: Option<PathBuf>,
    opened_text: String,
    actions_store: ActionsStore,
}

impl MyApp {
    pub fn new(icons: IconStore, opened_file: Option<PathBuf>, opened_text: String) -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let files = read_current_folder(&current_dir);
        let actions_store = ActionsStore::new();

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

// Реализация интерфейса
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let store = &self.actions_store;
        store.get_all_items();
        let items = store.items.get(ctx);

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

                for item in items.iter() {
                    ui.label(item); // теперь borrow ui безопасен
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
