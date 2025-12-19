use std::path::PathBuf;

use eframe::egui::{self};

pub fn code_editor<'t>(
    opened_file: Option<&PathBuf>,
    opened_text: &mut String,
    ctx: &egui::Context,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if let Some(path) = opened_file {
            ui.heading(path.file_name().unwrap().to_string_lossy());
            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add_sized(
                        [ui.available_width(), ui.available_height()],
                        egui::TextEdit::multiline(opened_text).font(egui::TextStyle::Monospace),
                    );
                });
        } else {
            ui.label("Файл не открыт");
        }
    });
}
