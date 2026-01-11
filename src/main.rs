use crate::{
    core::{
        app::MyApp,
        stores::{app_name_store::AppNameStore, icons::IconsInteractionsStore},
    },
    modules::plugins::run_test_plugin,
};

pub mod core;
pub mod modules;

const APP_NAME: &str = "riff";

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    let app_name_store = AppNameStore::new();

    run_test_plugin();

    eframe::run_native(
        APP_NAME,
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
