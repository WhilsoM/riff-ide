use std::cell::RefCell;
use std::rc::Rc;

use crate::core::enums::enums::{FileType, Icon};
use crate::core::lib::rsx::component::Children;
use crate::core::lib::rsx::component::Element;
use crate::core::models::Entry;
use crate::core::ui::ui_kit::StyleSheet;
use crate::core::ui::ui_kit::style::{FlexDirection, Style};
use crate::core::ui::ui_kit::{Image, SelectableLabel, Spacer, View};
use crate::core::utils::utils::read_current_folder;
use crate::modules::editor::stores::context::{get_file_interactions, get_icons};
use crate::modules::editor::stores::theme_store;
use crate::rsx;
use riff_rsx_macro::component;

#[component]
pub fn FileTreeItem(
    entry: Rc<RefCell<Entry>>,
    indent: usize,
    ctx: eframe::egui::Context,
) -> Element {
    let icons = get_icons();
    let theme = theme_store();
    let interactions = get_file_interactions();
    let entry_clone = entry.clone();

    let (icon_texture, name, ftype, path) = {
        let entry_borrowed = entry.borrow();
        let icon = match entry_borrowed.ftype {
            FileType::Folder if entry_borrowed.is_open => Icon::OpenFolder,
            FileType::Folder => Icon::Folder,
            _ => Icon::File,
        };
        let icon_texture = icons.get(&icon).id();
        let name = entry_borrowed
            .path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let ftype = entry_borrowed.ftype.clone();
        let path = entry_borrowed.path.clone();
        (icon_texture, name, ftype, path)
    };

    let click_handler = {
        let entry = entry_clone.clone();
        let path_clone = path.clone();
        let ftype_clone = ftype.clone();
        Rc::new(move || {
            let mut entry = entry.borrow_mut();
            match ftype_clone {
                FileType::Folder => {
                    entry.is_open = !entry.is_open;
                    if entry.is_open && entry.children.is_empty() {
                        entry.children = read_current_folder(&path_clone);
                    }
                }
                FileType::File => {
                    interactions.borrow_mut().handle_file_click(&path_clone);
                }
                _ => {}
            }
        })
    };

    let styles = StyleSheet::new().with(
        "container",
        Style::new()
            .padding_horizontal(10.0)
            .flex_direction(FlexDirection::Row),
    );

    let children_components: Vec<Element> = {
        let entry_borrowed = entry.borrow();
        if entry_borrowed.is_open && !entry_borrowed.children.is_empty() {
            entry_borrowed
                .children
                .iter()
                .map(|child| {
                    let child_ref = Rc::new(RefCell::new(child.clone()));
                    FileTreeItem(child_ref, indent + 1, ctx.clone())
                })
                .collect()
        } else {
            Vec::new()
        }
    };

    rsx! {
        View {
            align: "start".to_string(),
            justify: "start".to_string(),
            style: styles.get("container"),
            children: Children::Multiple({
                let mut ch = vec![
                    rsx! {
                        Spacer {
                            size: (indent * 12) as f32,
                        }
                    },
                    rsx! {
                        Image {
                            texture_id: Some(icon_texture),
                        }
                    },
                    rsx! {
                        SelectableLabel {
                            selected: false,
                            text: name.clone(),
                            text_color: Some(theme.text_primary.get(&ctx)),
                            hover_color: Some(theme.bg_hover.get(&ctx)),
                            on_click: Some(click_handler.clone()),
                        }
                    },
                ];
                ch.extend(children_components);
                ch
            }),
        }
    }
}
