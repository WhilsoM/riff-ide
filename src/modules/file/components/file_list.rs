use eframe::egui;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::core::lib::rsx::component::Component;
use crate::core::models::Entry;
use crate::core::stores::icons::IconsInteractionsStore;
use crate::modules::file::components::file_tree_item::FileTreeItem;
use crate::modules::file::stores::file_interactions::FileInteractionsStore;
use crate::modules::file::stores::theme::ThemeInteractionsStore;

fn sync_entry_recursive(original: &mut Entry, updated: &Entry) {
    original.is_open = updated.is_open;
    original.children = updated.children.clone();

    if !original.children.is_empty() && !updated.children.is_empty() {
        for (i, updated_child) in updated.children.iter().enumerate() {
            if i < original.children.len() {
                sync_entry_recursive(&mut original.children[i], updated_child);
            }
        }
    }
}

pub fn FileList(
    data: Rc<RefCell<Vec<Entry>>>,
    icons: Rc<IconsInteractionsStore>,
    interactions: Rc<RefCell<FileInteractionsStore>>,
    theme: Rc<ThemeInteractionsStore>,
) -> Rc<dyn Component> {
    Rc::new(crate::core::lib::rsx::component::ComponentWrapper::new(
        move |ui: &mut egui::Ui| {
            let entries = data.borrow();

            let mut entry_refs: Vec<(PathBuf, Rc<RefCell<Entry>>)> = Vec::new();

            for entry in entries.iter() {
                let entry_rc = Rc::new(RefCell::new(entry.clone()));
                let path = entry.path.clone();
                entry_refs.push((path, entry_rc.clone()));

                let tree_item = FileTreeItem(
                    entry_rc,
                    icons.clone(),
                    interactions.clone(),
                    theme.clone(),
                    0,
                );
                tree_item.render(ui);
            }

            drop(entries);
            let mut entries = data.borrow_mut();
            for (path, entry_rc) in entry_refs {
                if let Some(original_entry) = entries.iter_mut().find(|e| e.path == path) {
                    let updated_entry = entry_rc.borrow().clone();
                    sync_entry_recursive(original_entry, &updated_entry);
                }
            }
        },
    )) as Rc<dyn Component>
}
