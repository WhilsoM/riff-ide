use crate::core::lib::rsx::Children;
use crate::core::ui::ui_kit::{Text, View};
use crate::rsx;
use crate::{
    core::{
        lib::rsx::Element,
        ui::ui_kit::{Style, StyleSheet},
    },
    modules::editor::stores::theme_store,
};
use riff_rsx_macro::component;

#[component]
pub fn Settings(ctx: eframe::egui::Context) -> Element {
    let theme = theme_store();

    let settings_style = StyleSheet::new().with(
        "settings",
        Style::new()
            .padding(20.0)
            .background_color(theme.bg_main_100.get(&ctx))
            .width(400.0)
            .height(300.0)
            .border_radius(8.0)
            .border_width(1.0)
            .border_color(theme.border_primary.get(&ctx)),
    );

    rsx! {
        View {
            style: settings_style.get("settings"),
            children: {
                Text {
                    content: "SETTINGS".to_string(),
                }
            }
        }
    }
}
