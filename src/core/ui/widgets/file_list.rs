use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::enums::enums::{FileType, Icon, UiAction};
use crate::core::icons_store::IconStore;
use crate::core::lib::rsx::component::Component;
use crate::core::models::Entry;
use crate::core::ui::ui_kit::{Text, View};
use crate::core::utils::utils::read_current_folder;
use crate::rsx;

pub fn FileList(
    data: Rc<RefCell<Vec<Entry>>>,
    icons: Rc<IconStore>,
    on_item_click: Option<Rc<dyn Fn(&mut Entry) -> Option<UiAction>>>,
    pending_action: Rc<RefCell<Option<UiAction>>>,
) -> impl Component {
    crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut egui::Ui| {
        let mut entries = data.borrow_mut();
        for entry in entries.iter_mut() {
            if let Some(action) = render_entry(ui, entry, &icons, 0, &on_item_click) {
                *pending_action.borrow_mut() = Some(action);
            }
        }
    })
}

fn render_entry(
    ui: &mut egui::Ui,
    entry: &mut Entry,
    icons: &IconStore,
    indent: usize,
    on_click: &Option<Rc<dyn Fn(&mut Entry) -> Option<UiAction>>>,
) -> Option<UiAction> {
    let mut action = None;

    ui.horizontal(|ui| {
        ui.add_space((indent * 12) as f32);

        let icon = match entry.ftype {
            FileType::Folder if entry.is_open => Icon::OpenFolder,
            FileType::Folder => Icon::Folder,
            _ => Icon::File,
        };

        ui.image(icons.get(&icon));

        let name = entry.path.file_name().unwrap().to_string_lossy();

        if ui.selectable_label(false, name.clone()).clicked() {
            match entry.ftype {
                FileType::Folder => {
                    entry.is_open = !entry.is_open;
                    if entry.is_open && entry.children.is_empty() {
                        entry.children = read_current_folder(&entry.path);
                    }
                }
                FileType::File => {
                    if let Some(handler) = on_click {
                        action = handler(entry);
                    }
                }
                _ => {}
            }
        }
    });

    if entry.is_open {
        for child in &mut entry.children {
            if let Some(child_action) = render_entry(ui, child, icons, indent + 1, on_click) {
                action = Some(child_action);
            }
        }
    }

    action
}
