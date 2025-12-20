use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use eframe::egui;

use crate::core::core::ActionsStore;
use crate::core::ui::ui_kit::{
    render_central_panel, Button, CentralPanel, ScrollArea, Separator, Text, TextEdit, View,
};
use crate::modules::file::stores::theme::ThemeInteractionsStore;
use crate::rsx;

pub struct CodeEditor {
    opened_file: Option<PathBuf>,
    opened_text: Rc<RefCell<String>>,
    actions_store: Rc<RefCell<ActionsStore>>,
    theme: Rc<ThemeInteractionsStore>,
}

impl CodeEditor {
    pub fn new(
        opened_file: Option<PathBuf>,
        opened_text: Rc<RefCell<String>>,
        actions_store: Rc<RefCell<ActionsStore>>,
        theme: Rc<ThemeInteractionsStore>,
    ) -> Self {
        Self {
            opened_file,
            opened_text,
            actions_store,
            theme,
        }
    }

    pub fn render(&self, ctx: &egui::Context) {
        if let Some(path) = &self.opened_file {
            let increment_handler = {
                let store = Rc::clone(&self.actions_store);
                let ctx = ctx.clone();
                Rc::new(move || {
                    store.borrow().increment(&ctx);
                })
            };

            let counter = self.actions_store.borrow().counter.get(ctx);
            let counter_text = format!("Counter: {}", counter);
            let file_name = path.file_name().unwrap().to_string_lossy();
            let text_value = self.opened_text.clone();
            let theme_style_100 = self.theme.bg_main_100_style();
            let theme_style_200 = self.theme.bg_main_200_style();

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
                                    style: Some(theme_style_200),
                                    children: {
                                        Text {
                                            content: file_name.to_string(),
                                        };
                                        View {
                                            align: "end".to_string(),
                                            justify: "center".to_string(),
                                            children: {
                                                Text {
                                                    content: counter_text,
                                                };
                                                Button {
                                                    text: "➕".to_string(),
                                                    on_click: Some(increment_handler),
                                                }
                                            }
                                        }
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

            let theme_style_100 = self.theme.bg_main_100_style();
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
