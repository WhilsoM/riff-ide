use eframe::egui;

use crate::core::{
    enums::enums::UiAction, icons_store::IconStore, models::Entry, ui::draw_entry::draw_entry,
};

pub fn side_panel(
    files: &mut Vec<Entry>,
    icons: &IconStore,
    ctx: &egui::Context,
) -> Option<UiAction> {
    let mut pending_action = None;

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Панель файлов");

            for file in files.iter_mut() {
                if let Some(action) = draw_entry(ui, file, icons, 2) {
                    pending_action = Some(action);
                }
            }
        });

    pending_action
}
