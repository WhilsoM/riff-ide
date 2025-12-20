use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use eframe::egui;

use crate::core::core::ActionsStore;
use crate::core::lib::rsx::component::Component;
use crate::core::ui::ui_kit::{
    render_central_panel, Button, CentralPanel, ScrollArea, Separator, Text, TextEdit, View,
};
use crate::rsx;

pub fn code_editor<'t>(
    opened_file: Option<&PathBuf>,
    opened_text: &mut String,
    actions_store: &Rc<RefCell<ActionsStore>>,
    ctx: &egui::Context,
) {
    if let Some(path) = opened_file {
        let increment_handler = {
            let store = Rc::clone(actions_store);
            let ctx = ctx.clone();
            std::rc::Rc::new(move || {
                store.borrow().increment(&ctx);
            })
        };

        let counter = actions_store.borrow().counter.get(ctx);
        let counter_text = format!("Счетчик: {}", counter);
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let text_value = Rc::new(RefCell::new(opened_text.clone()));

        let editor_view = rsx! {
            CentralPanel {
                children: {
                    View {
                        align: "start".to_string(),
                        justify: "start".to_string(),
                        children: {
                            View {
                                align: "start".to_string(),
                                justify: "space-between".to_string(),
                                children: {
                                    Text {
                                        content: file_name.clone(),
                                    };
                                    View {
                                        align: "end".to_string(),
                                        justify: "center".to_string(),
                                        children: {
                                            Text {
                                                content: counter_text.clone(),
                                            };
                                            Button {
                                                text: "➕ Инкремент".to_string(),
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
                                        value: text_value.clone(),
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

        *opened_text = text_value.borrow().clone();
    } else {
        let hint_handler = std::rc::Rc::new(|| {
            println!("Подсказка: выберите файл из панели слева");
        });

        let empty_view = rsx! {
            CentralPanel {
                children: {
                    View {
                        align: "center".to_string(),
                        justify: "center".to_string(),
                        children: {
                            Text {
                                content: "Файл не открыт".to_string(),
                            };
                            Button {
                                text: "Открыть файл из панели слева".to_string(),
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
