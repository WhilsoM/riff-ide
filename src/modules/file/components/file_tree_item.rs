use std::cell::RefCell;
use std::rc::Rc;

use crate::core::enums::enums::{FileType, Icon};
use crate::core::lib::rsx::component::Component;
use crate::core::lib::rsx::component::ComponentWrapper;
use crate::core::models::Entry;
use crate::core::stores::icons::IconsInteractionsStore;
use crate::core::ui::ui_kit::StyleSheet;
use crate::core::ui::ui_kit::style::{FlexDirection, Style};
use crate::core::ui::ui_kit::{Image, SelectableLabel, Spacer, View};
use crate::core::utils::utils::read_current_folder;
use crate::modules::file::stores::file_interactions::FileInteractionsStore;
use crate::modules::file::stores::theme::ThemeInteractionsStore;
use crate::rsx;

#[allow(non_snake_case)]
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

    let (icon_texture, name, ftype, path) = {
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
        let ftype = entry_borrowed.ftype.clone();
        let path = entry_borrowed.path.clone();
        (icon_texture, name, ftype, path)
    };

    let click_handler = {
        let entry = entry_clone.clone();
        let interactions = interactions_clone.clone();
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
                    let mut interactions = interactions.borrow_mut();
                    interactions.handle_file_click(&path_clone);
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

    let (children_components, child_refs): (Vec<Rc<dyn Component>>, Vec<Rc<RefCell<Entry>>>) = {
        let entry_borrowed = entry.borrow();
        if entry_borrowed.is_open && !entry_borrowed.children.is_empty() {
            entry_borrowed
                .children
                .iter()
                .map(|child| {
                    let child_ref = Rc::new(RefCell::new(child.clone()));
                    let component = FileTreeItem(
                        child_ref.clone(),
                        icons_clone.clone(),
                        interactions_clone.clone(),
                        theme_clone.clone(),
                        indent + 1,
                    );
                    (component, child_ref)
                })
                .unzip()
        } else {
            (Vec::new(), Vec::new())
        }
    };

    Rc::new(ComponentWrapper::new({
        let entry = entry_clone.clone();
        let child_refs = child_refs.clone();
        let children_components = children_components.clone();

        move |ui: &mut eframe::egui::Ui| {
            rsx! {
                View {
                    align: "start".to_string(),
                    justify: "start".to_string(),
                    style: styles.get("container"),
                    children: {
                        Spacer {
                            size: (indent * 12) as f32,
                        };
                        Image {
                            texture_id: Some(icon_texture),
                        };
                        SelectableLabel {
                            selected: false,
                            text: name.clone(),
                            text_color: Some(theme_clone.text_primary),
                            hover_color: Some(theme_clone.bg_hover),
                            on_click: Some(click_handler.clone()),
                        }
                    }
                }
            }
            .render(ui);

            for child in &children_components {
                child.render(ui);
            }

            let mut entry = entry.borrow_mut();
            if !child_refs.is_empty() && !entry.children.is_empty() {
                for (i, child_ref) in child_refs.iter().enumerate() {
                    if i < entry.children.len() {
                        entry.children[i] = child_ref.borrow().clone();
                    }
                }
            }
        }
    })) as Rc<dyn Component>
}
