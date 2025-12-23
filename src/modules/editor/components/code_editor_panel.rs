use crate::core::lib::rsx::component::Element;
use crate::core::ui::ui_kit::{Button, CentralPanel, ScrollArea, Text, TextEdit, View};
use crate::modules::editor::components::TabsBar;
use crate::modules::editor::stores::editor::editor_interactions::editor_interactions_store;
use crate::modules::editor::stores::theme_store;
use crate::rsx;
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
        use std::rc::Rc;
        let hint_handler = Rc::new(|| {
            println!("Select a file from the explorer");
        });

        rsx! {
            CentralPanel {
                children: {
                    View {
                        align: "center".to_string(),
                        justify: "center".to_string(),
                        style: Some(theme.bg_main_100_style(&ctx)),
                        children: {
                            Text {
                                content: "No file open".to_string(),
                            };
                            Button {
                                text: "Select a file from explorer".to_string(),
                                on_click: Some(hint_handler),
                            }
                        }
                    }
                }
            }
        }
    }
}
