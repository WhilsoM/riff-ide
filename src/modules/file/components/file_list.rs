use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::lib::rsx::component::Component;
use crate::core::models::Entry;
use crate::core::stores::icons::IconsInteractionsStore;
use crate::modules::file::components::file_tree_item::FileTreeItem;
use crate::modules::file::stores::file_interactions::FileInteractionsStore;
use crate::modules::file::stores::theme::ThemeInteractionsStore;

pub fn FileList(
    data: Rc<RefCell<Vec<Entry>>>,
    icons: Rc<IconsInteractionsStore>,
    interactions: Rc<RefCell<FileInteractionsStore>>,
    theme: Rc<ThemeInteractionsStore>,
) -> Rc<dyn Component> {
    Rc::new(crate::core::lib::rsx::component::ComponentWrapper::new(
        move |ui: &mut egui::Ui| {
            let mut entries = data.borrow_mut();

            for entry in entries.iter_mut() {
                let entry_rc = Rc::new(RefCell::new(entry.clone()));
                let tree_item = FileTreeItem(
                    entry_rc.clone(),
                    icons.clone(),
                    interactions.clone(),
                    theme.clone(),
                    0,
                );
                tree_item.render(ui);

                *entry = entry_rc.borrow().clone();
            }
        },
    )) as Rc<dyn Component>
}
