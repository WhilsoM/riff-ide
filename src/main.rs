use crate::core::core::{IconStore, MyApp};

pub mod core;
pub mod modules;

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
