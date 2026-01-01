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

    // let settings_style = StyleSheet::new().with(
    //     "main_settings",
    //     Style::new()
    //         .padding(20.0)
    //         .background_color(theme.bg_main_100.get(&ctx))
    //         .width(400.0)
    //         .height(300.0)
    //         .border_radius(8.0)
    //         .border_width(1.0)
    //         .border_color(theme.border_primary.get(&ctx)),
    // );

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
