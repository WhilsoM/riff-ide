use std::cell::RefCell;
use std::rc::Rc;

use crate::core::enums::enums::{FileType, Icon};
use crate::core::lib::rsx::component::Children;

use crate::core::types::types::{Element, EntryRc};
use crate::core::ui::ui_kit::StyleSheet;
use crate::core::ui::ui_kit::style::{Align, FlexDirection, Style};
use crate::core::ui::ui_kit::{Image, SelectableLabel, View};
use crate::core::utils::utils::read_current_folder;
use crate::modules::editor::stores::context::{get_file_interactions, get_icons};
use crate::modules::editor::stores::theme_store;
use crate::{on_click, rsx};
use riff_rsx_macro::component;

#[component]
pub fn FileTreeItem(entry: EntryRc, indent: usize, ctx: eframe::egui::Context) -> Element {
    let icons = get_icons();
    let theme = theme_store();
    let interactions = get_file_interactions();

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

    fn click_handler(
        entry: EntryRc,
        path: std::path::PathBuf,
        ftype: FileType,
        interactions: Rc<RefCell<crate::modules::editor::stores::FileInteractionsStore>>,
        ctx_for_handler: eframe::egui::Context,
    ) {
        let mut entry = entry.borrow_mut();

        match ftype {
            FileType::Folder => {
                entry.is_open = !entry.is_open;

                if entry.is_open && entry.children.is_empty() {
                    entry.children = read_current_folder(&path);
                    println!(
                        "ЗАГРУЖЕННЫЕ ДЕТИ ДЛЯ ПАПКИ {:?}: {:?}, {}",
                        path, entry.children, entry.is_open
                    );
                }
            }
            FileType::File => {
                interactions
                    .borrow_mut()
                    .handle_file_click(&ctx_for_handler, &path);
            }
            _ => {}
        }
    }

    let styles = StyleSheet::new()
        .with(
            "container",
            Style::new()
                .padding_horizontal(10.0)
                .flex_direction(FlexDirection::Column),
        )
        .with(
            "item",
            Style::new()
                .flex_direction(FlexDirection::Row)
                .align(Align::Center)
                .padding_vertical(1.0),
        );

    let children_components: Vec<Element> = {
        let entry_borrowed = entry.borrow();

        if entry_borrowed.is_open {
            entry_borrowed
                .children
                .iter()
                .map(|child_rc| FileTreeItem(child_rc.clone(), indent + 1, ctx.clone()))
                .collect()
        } else {
            vec![]
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
                      View {
                        style: styles.get("item"),
                        children: {
                          Image {
                            texture_id: Some(icon_texture),
                          };

                          SelectableLabel {
                            selected: false,
                            text: name.clone(),
                            text_color: Some(theme.text_primary.get(&ctx)),
                            hover_color: Some(theme.bg_hover.get(&ctx)),
                            on_click: Some(on_click!(
                                click_handler,
                                entry.clone(),
                                path.clone(),
                                ftype.clone(),
                                interactions.clone(),
                                ctx.clone()
                            )),
                          };
                        }
                      }
                    },
                ];
                ch.extend(children_components);
                ch
            }),
        }
    }
}
