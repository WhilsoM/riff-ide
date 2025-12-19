use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use eframe::egui;

fn main() -> eframe::Result<()> {
    // Настройки окна
    let native_options = eframe::NativeOptions::default();

    // Запуск приложения
    eframe::run_native(
        "My IDE",
        native_options,
        Box::new(|cc| {
            let icons = IconStore::new(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(icons, None, String::new())))
        }),
    )
}

// Структура приложения
struct MyApp {
    current_dir: PathBuf,
    files: Vec<Entry>,
    icons: IconStore,
    opened_file: Option<PathBuf>,
    opened_text: String,
}

impl MyApp {
    fn new(icons: IconStore, opened_file: Option<PathBuf>, opened_text: String) -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        let files = read_current_folder(&current_dir);

        Self {
            current_dir,
            files,
            icons,
            opened_file,
            opened_text,
        }
    }

    fn open_file(&mut self, path: &Path) {
        if let Ok(text) = fs::read_to_string(path) {
            self.opened_file = Some(path.to_path_buf());
            self.opened_text = text
        }
    }
}

// Реализация интерфейса
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

fn draw_entry(
    ui: &mut egui::Ui,
    entry: &mut Entry,
    icons: &IconStore,
    indent: usize,
) -> Option<UiAction> {
    let mut action = None;
    ui.horizontal(|ui| {
        ui.add_space((indent * 12) as f32);

        let icon = match entry.ftype {
            FileType::Folder if entry.is_open => Icon::OpenFolder,
            FileType::Folder => Icon::Folder,
            _ => Icon::File,
        };

        ui.image(icons.get(&icon));

        let name = entry.path.file_name().unwrap().to_string_lossy();

        if ui.selectable_label(false, name).clicked() {
            match entry.ftype {
                FileType::Folder => {
                    entry.is_open = !entry.is_open;

                    if entry.is_open && entry.children.is_empty() {
                        entry.children = read_current_folder(&entry.path);
                    }
                }
                FileType::File => {
                    action = Some(UiAction::OpenFile(entry.path.clone()));
                }
                _ => {}
            }
        }
    });

    // 🔥 ВАЖНО: если папка открыта — рисуем детей
    if entry.is_open {
        for child in &mut entry.children {
            if let Some(child_action) = draw_entry(ui, child, icons, indent + 1) {
                action = Some(child_action);
            }
        }
    }

    action
}

fn load_children(entry: &mut Entry) {
    if entry.ftype != FileType::Folder {
        return;
    }

    if !entry.children.is_empty() {
        println!("ДЕТИ ЕСТЬ? {}", entry.children.is_empty());
        return;
    }

    entry.children = read_current_folder(&entry.path);
}

fn read_current_folder(path: &PathBuf) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    if let Ok(dir_entries) = fs::read_dir(path) {
        for entry in dir_entries.flatten() {
            let file_name = entry.file_name();

            if let Ok(name) = file_name.into_string() {
                if name == ".git" || name == ".DS_Store" {
                    continue;
                }

                entries.push(Entry {
                    path: entry.path(),
                    ftype: get_file_type(&entry.path()),
                    is_open: false,
                    children: Vec::new(),
                });
            }
        }
    }

    entries.sort_by(|a, b| match (&a.ftype, &b.ftype) {
        (FileType::Folder, FileType::File) => std::cmp::Ordering::Less,
        (FileType::File, FileType::Folder) => std::cmp::Ordering::Greater,
        _ => a.path.file_name().cmp(&b.path.file_name()),
    });

    entries
}

fn file_type_label(ftype: &FileType) -> &str {
    match ftype {
        FileType::Folder => "Папка",
        FileType::File => "Файл",
        FileType::Symlink => "Символическая ссылка",
    }
}

fn get_file_type(path: &Path) -> FileType {
    if let Ok(metadata) = fs::metadata(path) {
        let ft = metadata.file_type();
        if ft.is_dir() {
            return FileType::Folder;
        } else if ft.is_file() {
            return FileType::File;
        } else if ft.is_symlink() {
            return FileType::Symlink;
        }
    }
    FileType::File // по умолчанию, если не получилось определить
}
#[derive(PartialEq)]
enum FileType {
    Folder,
    File,
    Symlink,
}

struct Entry {
    path: PathBuf,
    ftype: FileType,
    is_open: bool,
    children: Vec<Entry>,
}
#[derive(Hash, Eq, PartialEq)]
enum Icon {
    Folder,
    OpenFolder,
    File,
    Rust,
}

struct IconStore {
    icons: HashMap<Icon, egui::TextureHandle>,
}

impl IconStore {
    fn new(ctx: &egui::Context) -> Self {
        let mut icons = HashMap::new();

        icons.insert(
            Icon::Folder,
            load_icon(ctx, "folder", include_bytes!("../assets/icons/folder.png")),
        );

        icons.insert(
            Icon::OpenFolder,
            load_icon(
                ctx,
                "open_folder",
                include_bytes!("../assets/icons/folder-open.png"),
            ),
        );

        icons.insert(
            Icon::File,
            load_icon(ctx, "file", include_bytes!("../assets/icons/file-text.png")),
        );

        icons.insert(
            Icon::Rust,
            load_icon(ctx, "rust", include_bytes!("../assets/icons/rust-icon.png")),
        );

        Self { icons }
    }

    fn get(&self, icon: &Icon) -> &egui::TextureHandle {
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
enum UiAction {
    OpenFile(PathBuf),
}
