use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::core::ui::ui_kit::{
    Button, CentralPanel, ScrollArea, Separator, Style, StyleSheet, Text, TextEdit, View,
    render_central_panel,
};

use eframe::egui;

use crate::modules::editor::stores::theme_store;
use crate::rsx;

pub struct CodeEditor {
    opened_file: Option<PathBuf>,
    opened_text: Rc<RefCell<String>>,
}

impl CodeEditor {
    pub fn new(opened_file: Option<PathBuf>, opened_text: Rc<RefCell<String>>) -> Self {
        Self {
            opened_file,
            opened_text,
        }
    }

    pub fn render(&self, ctx: &egui::Context) {
        if let Some(path) = &self.opened_file {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let text_value = self.opened_text.clone();
            let theme = theme_store();
            let theme_style_100 = theme.bg_main_100_style(ctx);
            let theme_style_text = theme.text_primary_style(ctx);

            println!("Rendering code editor text: {:?}", text_value);

            let style = StyleSheet::new().with(
                "file_container",
                Style::new()
                    .padding(10.0)
                    .background_color(theme.bg_main_200.get(ctx)),
            );

            let editor_view = rsx! {
                CentralPanel {
                    children: {
                        View {
                            align: "start".to_string(),
                            justify: "start".to_string(),
                            style: Some(theme_style_100),
                            children: {
                                View {
                                    align: "start".to_string(),
                                    justify: "space-between".to_string(),
                                    style: style.get("file_container"),
                                    children: {
                                        Text {
                                            content: file_name.to_string(),
                                            style: Some(theme_style_text),
                                        };
                                    }
                                };
                                Separator {};
                                ScrollArea {
                                    auto_shrink: Some((false, false)),
                                    children: {
                                        TextEdit {
                                            value: text_value,
                                            multiline: true,
                                            font: Some("monospace".to_string()),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };

            render_central_panel(ctx, editor_view);
        } else {
            let hint_handler = Rc::new(|| {
                println!("Select a file from the explorer");
            });

            let theme = theme_store();
            let theme_style_100 = theme.bg_main_100_style(ctx);
            let empty_view = rsx! {
                CentralPanel {
                    children: {
                        View {
                            align: "center".to_string(),
                            justify: "center".to_string(),
                            style: Some(theme_style_100),
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
            };

            render_central_panel(ctx, empty_view);
        }
    }
}
