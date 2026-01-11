use crate::{
    core::ui::ui_kit::{Text, View},
    rsx,
};
use eframe::egui;
use riff_rsx_macro::component;

use crate::{
    core::{
        lib::rsx::Element,
        stores::global_store::global_store,
        ui::ui_kit::{Style, StyleSheet},
    },
    modules::editor::stores::theme_store,
};

#[component]
pub fn Settings(ctx: egui::Context) -> Element {
    let theme = theme_store();

    let settings_style = StyleSheet::new().with("main_settings", Style::new());

    rsx! {
       View {
              align: "center".to_string(),
              justify: "center".to_string(),
              style: Some(theme.bg_main_100_style(&ctx)),
              children: {
                  Text {
                      content: "No file open".to_string(),
                  };
              }
          }
    }
}
