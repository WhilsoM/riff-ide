use crate::core::lib::rsx::component::Element;
use crate::core::ui::ui_kit::{Button, Style, StyleSheet, View};
use crate::modules::editor::components::BottomPanel;
use crate::modules::editor::stores::theme_store;
use crate::rsx;
use riff_rsx_macro::component;
use std::rc::Rc;

#[component]
pub fn Navbar(ctx: eframe::egui::Context) -> Element {
    let theme = theme_store();
    let settings_handler = Rc::new(|| {
        println!("Settings clicked");
    });

    let navbar_style = StyleSheet::new().with(
        "navbar",
        Style::new()
            .padding(10.0)
            .padding_horizontal(20.0)
            .background_color(theme.bg_main_200.get(&ctx)),
    );

    rsx! {
        BottomPanel {
            id: "navbar".to_string(),
            children: {
                View {
                    align: "center".to_string(),
                    justify: "flex-end".to_string(),
                    style: navbar_style.get("navbar"),
                    children: {
                        Button {
                            text: "Settings".to_string(),
                            on_click: Some(settings_handler),
                        }
                    }
                }
            }
        }
    }
}
