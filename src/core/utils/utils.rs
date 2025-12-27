use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use eframe::egui;

use crate::core::{
    enums::enums::FileType,
    models::{Entry, EntryRc},
};

// СКОРЕЕ ВСЕГО ЧТО ТО С ЭТОЙ ФУНКЦИЕЙ ИЛИ ОТОБРАЖЕНИЕМ В КОМПОНЕНТЕ ПОСМОТРЕТЬ fileList and FileTreeItem
/// читать текущую директорию
/// TODO: сделать функцию выбирание папки для чтения
pub fn read_current_folder(path: &PathBuf) -> Vec<EntryRc> {
    let mut entries: Vec<EntryRc> = Vec::new();

    println!("ЧТЕНИЕ ПАПКИ: {:?}", path);

    if let Ok(dir_entries) = fs::read_dir(path) {
        for entry in dir_entries.flatten() {
            let file_name = entry.file_name();

            if let Ok(name) = file_name.into_string() {
                // todo: если файл скрыт то не показывать его
                if name == ".git" || name == ".DS_Store" {
                    continue;
                }
                let entry_rc = Rc::new(RefCell::new(Entry {
                    path: entry.path(),
                    ftype: get_file_type(&entry.path()),
                    is_open: false,
                    children: Vec::new(), // 👈 тоже Rc внутри
                }));

                entries.push(entry_rc);
            }
        }
    }

    entries.sort_by(|a, b| {
        let a = a.borrow();
        let b = b.borrow();

        match (&a.ftype, &b.ftype) {
            (FileType::Folder, FileType::File) => std::cmp::Ordering::Less,
            (FileType::File, FileType::Folder) => std::cmp::Ordering::Greater,
            _ => a.path.file_name().cmp(&b.path.file_name()),
        }
    });
    println!("ЗАГРУЖЕННЫЕ ЭНТРИ: {:?}", entries);

    entries
}

/// получить текст из типа файла
pub fn file_type_label(ftype: &FileType) -> &str {
    match ftype {
        FileType::Folder => "Папка",
        FileType::File => "Файл",
        FileType::Symlink => "Символическая ссылка",
    }
}

/// загрузка детей (дерево)
pub fn load_children(entry: &mut Entry) {
    if entry.ftype != FileType::Folder {
        return;
    }

    if !entry.children.is_empty() {
        println!("ДЕТИ ЕСТЬ? {}", entry.children.is_empty());
        return;
    }

    entry.children = read_current_folder(&entry.path);
}

/// получение типа файла
pub fn get_file_type(path: &Path) -> FileType {
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
    FileType::File
}

/// функция для загрузки иконок (icons_store.rs)
pub fn load_icon(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
    let image = image::load_from_memory(bytes).unwrap().to_rgba8();

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    ctx.load_texture(name, color_image, Default::default())
}
