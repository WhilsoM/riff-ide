use crate::core::types::types::Element;
use crate::core::ui::ui_kit::{
    Button, CentralPanel, ScrollArea, Style, StyleSheet, Text, TextEdit, View,
};
use crate::modules::editor::components::TabsBar;
use crate::modules::editor::stores::editor::editor_interactions::editor_interactions_store;
use crate::modules::editor::stores::theme_store;
use crate::{on_click, rsx};
use egui::Color32;
use riff_rsx_macro::component;

#[component]
pub fn CodeEditorPanel(ctx: eframe::egui::Context) -> Element {
    let editor_interactions = editor_interactions_store();
    let theme = theme_store();
    let current_path = editor_interactions.get_current_tab_path(&ctx);
    let text_value = editor_interactions.get_current_tab_text_ref(&ctx);

    if let (Some(_path), Some(text_ref)) = (current_path, text_value) {
        rsx! {
            CentralPanel {
                children: {
                    View {
                        align: "start".to_string(),
                        justify: "start".to_string(),
                        style: Some(theme.bg_main_100_style(&ctx)),
                        children: {
                            TabsBar(ctx.clone());
                            ScrollArea {
                                auto_shrink: Some((false, false)),
                                children: {
                                    TextEdit {
                                        value: text_ref.clone(),
                                        multiline: true,
                                        font: Some("monospace".to_string()),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        fn hint_handler() {
            println!("Select a file from the explorer");
        }

        let style = StyleSheet::new().with(
            "no_file_open",
            Style::new()
                .flex(1)
                .height(f32::INFINITY)
                .background_color(Color32::from_rgb(25, 25, 25)),
        );

        rsx! {
            CentralPanel {
                children: {
                    View {
                        align: "center".to_string(),
                        justify: "center".to_string(),
                        style: style.get("no_file_open"),
                        children: {
                            Text {
                                content: "No file open".to_string(),
                            };
                            Button {
                                text: "Select a file from explorer".to_string(),
                                on_click: Some(on_click!(hint_handler)),
                            }
                        }
                    }
                }
            }
        }
    }
}
