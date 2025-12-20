use std::cell::RefCell;
use std::rc::Rc;

use crate::core::enums::enums::{FileType, Icon};
use crate::core::lib::rsx::component::Component;
use crate::core::lib::rsx::component::ComponentWrapper;
use crate::core::models::Entry;
use crate::core::stores::icons::IconsInteractionsStore;
use crate::core::ui::ui_kit::style::{FlexDirection, Style};
use crate::core::ui::ui_kit::StyleSheet;
use crate::core::ui::ui_kit::{Image, SelectableLabel, Spacer, View};
use crate::core::utils::utils::read_current_folder;
use crate::modules::file::stores::file_interactions::FileInteractionsStore;
use crate::modules::file::stores::theme::ThemeInteractionsStore;
use crate::rsx;

pub fn FileTreeItem(
    entry: Rc<RefCell<Entry>>,
    icons: Rc<IconsInteractionsStore>,
    interactions: Rc<RefCell<FileInteractionsStore>>,
    theme: Rc<ThemeInteractionsStore>,
    indent: usize,
) -> Rc<dyn Component> {
    let icons_clone = icons.clone();
    let interactions_clone = interactions.clone();
    let theme_clone = theme.clone();
    let entry_clone = entry.clone();

    let (icon_texture, name, is_open, ftype, path, children_refs) = {
        let entry_borrowed = entry.borrow();
        let icon = match entry_borrowed.ftype {
            FileType::Folder if entry_borrowed.is_open => Icon::OpenFolder,
            FileType::Folder => Icon::Folder,
            _ => Icon::File,
        };
        let icon_texture = icons_clone.get(&icon).id();
        let name = entry_borrowed
            .path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let is_open = entry_borrowed.is_open;
        let ftype = entry_borrowed.ftype.clone();
        let path = entry_borrowed.path.clone();
        let children_refs: Vec<Rc<RefCell<Entry>>> = entry_borrowed
            .children
            .iter()
            .map(|child| Rc::new(RefCell::new(child.clone())))
            .collect();
        (icon_texture, name, is_open, ftype, path, children_refs)
    };

    let click_handler = {
        let entry = entry_clone.clone();
        let interactions = interactions_clone.clone();
        let path_clone = path.clone();
        Rc::new(move || {
            let mut entry = entry.borrow_mut();
            match ftype {
                FileType::Folder => {
                    entry.is_open = !entry.is_open;
                    if entry.is_open && entry.children.is_empty() {
                        entry.children = read_current_folder(&path_clone);
                    }
                }
                FileType::File => {
                    let mut interactions = interactions.borrow_mut();
                    interactions.handle_file_click(&path_clone);
                }
                _ => {}
            }
        })
    };

    let row_style = Style::default().flex_direction(FlexDirection::Row);

    let children_components: Vec<Rc<dyn Component>> = if is_open {
        children_refs
            .into_iter()
            .map(|child_ref| {
                FileTreeItem(
                    child_ref,
                    icons_clone.clone(),
                    interactions_clone.clone(),
                    theme_clone.clone(),
                    indent + 1,
                )
            })
            .collect()
    } else {
        Vec::new()
    };

    fn render_children(children: Vec<Rc<dyn Component>>) -> Rc<dyn Component> {
        Rc::new(ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
            for child in &children {
                child.render(ui);
            }
        })) as Rc<dyn Component>
    }

    let style = StyleSheet::new()
        .with(
            "container",
            Style::new()
                .padding_horizontal(8.0)
                .background_color(theme_clone.bg_main_200),
        )
        .with(
            "row",
            Style::new()
                .flex_direction(FlexDirection::Row)
                .padding_horizontal(8.0)
                .background_color(theme_clone.bg_main_200),
        );

    rsx! {
        View {
            align: "start".to_string(),
            justify: "start".to_string(),
            style: style.get("container"),
            children: {
                View {
                    align: "start".to_string(),
                    justify: "start".to_string(),
                    style: Some(Rc::new(row_style)),
                    children: {
                        Spacer {
                            size: (indent * 12) as f32,
                        };
                        Image {
                            texture_id: Some(icon_texture),
                        };
                        SelectableLabel {
                            selected: false,
                            text: name,
                            text_color: Some(theme_clone.text_primary),
                            hover_color: Some(theme_clone.bg_hover),
                            on_click: Some(click_handler),
                        }
                    }
                };
                render_children(children_components)
            }
        }
    }
}
