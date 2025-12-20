use crate::core::{
    core::MyApp,
    stores::{app_name_store::AppNameStore, icons::IconsInteractionsStore},
};

pub mod core;
pub mod modules;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    let app_name_store = AppNameStore::new();

    eframe::run_native(
        "riff",
        native_options,
        Box::new(move |cc| {
            let icons = IconsInteractionsStore::new(&cc.egui_ctx);

            Ok(Box::new(MyApp::new(
                icons,
                None,
                String::new(),
                app_name_store.clone(),
            )))
        }),
    )
}
