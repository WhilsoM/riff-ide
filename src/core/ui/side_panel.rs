use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::lib::rsx::component::Component;
use crate::core::ui::ui_kit::{Button, Separator, StyleSheet, Text, View};
use crate::core::ui::widgets::{render_side_panel, FileList, SidePanel};
use crate::core::{enums::enums::UiAction, icons_store::IconStore, models::Entry};
use crate::rsx;

pub fn side_panel(
    files: &mut Vec<Entry>,
    icons: Rc<IconStore>,
    ctx: &egui::Context,
) -> Option<UiAction> {
    let files_rc = Rc::new(RefCell::new(files.clone()));
    let pending_action = Rc::new(RefCell::new(None::<UiAction>));

    let on_file_click = {
        let files_clone = Rc::clone(&files_rc);
        Rc::new(move |entry: &mut Entry| -> Option<UiAction> {
            match entry.ftype {
                crate::core::enums::enums::FileType::File => {
                    Some(UiAction::OpenFile(entry.path.clone()))
                }
                _ => None,
            }
        })
    };

    let refresh_handler = std::rc::Rc::new(|| {
        println!("Кнопка 'Обновить' нажата!");
    });

    let new_file_handler = std::rc::Rc::new(|| {
        println!("Кнопка 'Новый файл' нажата!");
    });

    let s = StyleSheet::new();

    let panel_view = rsx! {
        SidePanel("left_panel", true, rsx! {
            View {
                align: "start".to_string(),
                justify: "start".to_string(),
                children: {
                    Text {
                        content: "Панель файлов".to_string(),
                    };
                    Separator {};
                    View {
                        align: "start".to_string(),
                        justify: "start".to_string(),
                        style: Some(s.button_container.clone()),
                        children: {
                            Button {
                                text: "Обновить".to_string(),
                                on_click: Some(refresh_handler),
                            };
                            Button {
                                text: "Новый файл".to_string(),
                                on_click: Some(new_file_handler),
                            }
                        }
                    };
                    Separator {};
                    FileList(files_rc.clone(), icons.clone(), Some(on_file_click), pending_action.clone())
                }
            }
        })
    };

    render_side_panel(ctx, "left_panel", true, panel_view);

    *files = files_rc.borrow().clone();

    pending_action.borrow_mut().take()
}
