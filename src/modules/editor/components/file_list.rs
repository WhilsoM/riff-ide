use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::core::lib::rsx::component::Children;
use crate::core::lib::rsx::component::Element;
use crate::core::models::Entry;
use crate::core::ui::ui_kit::View;
use crate::modules::editor::components::file_tree_item::FileTreeItem;
use crate::modules::editor::stores::context::get_files;
use crate::rsx;
use riff_rsx_macro::component;

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

#[component]
pub fn FileList(ctx: eframe::egui::Context) -> Element {
    let files = get_files();
    let entries = files.borrow();

    let mut entry_refs: Vec<(PathBuf, Rc<RefCell<Entry>>)> = Vec::new();
    let mut children: Vec<Element> = Vec::new();

    for entry in entries.iter() {
        let entry_rc = Rc::new(RefCell::new(entry.clone()));
        let path = entry.path.clone();
        entry_refs.push((path, entry_rc.clone()));

        let tree_item = FileTreeItem(entry_rc, 0, ctx.clone());
        children.push(tree_item);
    }

    drop(entries);
    let mut entries = files.borrow_mut();
    for (path, entry_rc) in entry_refs {
        if let Some(original_entry) = entries.iter_mut().find(|e| e.path == path) {
            let updated_entry = entry_rc.borrow().clone();
            sync_entry_recursive(original_entry, &updated_entry);
        }
    }

    rsx! {
        View {
            align: "start".to_string(),
            justify: "start".to_string(),
            children: Children::Multiple(children),
        }
    }
}
