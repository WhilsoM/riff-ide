use eframe::egui;

use crate::core::{
    core::IconStore,
    enums::enums::{FileType, Icon, UiAction},
    models::Entry,
    utils::utils::read_current_folder,
};

pub fn draw_entry(
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

    if entry.is_open {
        for child in &mut entry.children {
            if let Some(child_action) = draw_entry(ui, child, icons, indent + 1) {
                action = Some(child_action);
            }
        }
    }

    action
}
