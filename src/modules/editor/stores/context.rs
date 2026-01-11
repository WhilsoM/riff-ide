use std::cell::RefCell;
use std::rc::Rc;

use crate::core::stores::global_store::GlobalStore;
use crate::core::stores::icons::IconsInteractionsStore;
use crate::core::types::types::EntryRc;
use crate::modules::editor::stores::hotkeys::HotkeysInteractionsStore;
use crate::modules::editor::stores::{
    EditorInteractionsStore, FileActionsStore, FileInteractionsStore, ThemeInteractionsStore,
};

thread_local! {
    static EDITOR_INTERACTIONS: RefCell<Option<Rc<RefCell<EditorInteractionsStore>>>> = RefCell::new(None);
    static THEME: RefCell<Option<Rc<ThemeInteractionsStore>>> = RefCell::new(None);
    static FILE_INTERACTIONS: RefCell<Option<Rc<RefCell<FileInteractionsStore>>>> = RefCell::new(None);
    static FILE_ACTIONS: RefCell<Option<Rc<RefCell<FileActionsStore>>>> = RefCell::new(None);
    static ICONS: RefCell<Option<Rc<IconsInteractionsStore>>> = RefCell::new(None);
    static FILES: RefCell<Option<Rc<RefCell<Vec<EntryRc>>>>> = RefCell::new(None);
    static HOTKEYS_INTERACTIONS: RefCell<Option<Rc<RefCell<HotkeysInteractionsStore>>>> = RefCell::new(None);
    static GLOBAL_STORE: RefCell<Option<Rc<RefCell<GlobalStore>>>> = RefCell::new(None);
}

pub fn set_editor_interactions(store: Rc<RefCell<EditorInteractionsStore>>) {
    EDITOR_INTERACTIONS.with(|s| *s.borrow_mut() = Some(store));
}

pub fn get_editor_interactions() -> Rc<RefCell<EditorInteractionsStore>> {
    EDITOR_INTERACTIONS.with(|s| {
        s.borrow()
            .as_ref()
            .expect("EditorInteractionsStore not initialized")
            .clone()
    })
}

pub fn set_theme(store: Rc<ThemeInteractionsStore>) {
    THEME.with(|s| *s.borrow_mut() = Some(store));
}

pub fn get_theme() -> Rc<ThemeInteractionsStore> {
    THEME.with(|s| {
        s.borrow()
            .as_ref()
            .expect("ThemeInteractionsStore not initialized")
            .clone()
    })
}

pub fn set_file_interactions(store: Rc<RefCell<FileInteractionsStore>>) {
    FILE_INTERACTIONS.with(|s| *s.borrow_mut() = Some(store));
}

pub fn get_file_interactions() -> Rc<RefCell<FileInteractionsStore>> {
    FILE_INTERACTIONS.with(|s| {
        s.borrow()
            .as_ref()
            .expect("FileInteractionsStore not initialized")
            .clone()
    })
}

pub fn set_file_actions(store: Rc<RefCell<FileActionsStore>>) {
    FILE_ACTIONS.with(|s| *s.borrow_mut() = Some(store));
}

pub fn get_file_actions() -> Rc<RefCell<FileActionsStore>> {
    FILE_ACTIONS.with(|s| {
        s.borrow()
            .as_ref()
            .expect("FileActionsStore not initialized")
            .clone()
    })
}

pub fn set_icons(store: Rc<IconsInteractionsStore>) {
    ICONS.with(|s| *s.borrow_mut() = Some(store));
}

pub fn get_icons() -> Rc<IconsInteractionsStore> {
    ICONS.with(|s| {
        s.borrow()
            .as_ref()
            .expect("IconsInteractionsStore not initialized")
            .clone()
    })
}

pub fn set_files(files: Rc<RefCell<Vec<EntryRc>>>) {
    FILES.with(|s| *s.borrow_mut() = Some(files));
}

pub fn get_files() -> Rc<RefCell<Vec<EntryRc>>> {
    FILES.with(|s| s.borrow().as_ref().expect("Files not initialized").clone())
}

pub fn get_hotkeys_interactions() -> Rc<RefCell<HotkeysInteractionsStore>> {
    HOTKEYS_INTERACTIONS.with(|s| {
        s.borrow()
            .as_ref()
            .expect("hotkeys interactions not initialized")
            .clone()
    })
}

pub fn get_global_store() -> Rc<RefCell<GlobalStore>> {
    GLOBAL_STORE.with(|s| {
        s.borrow()
            .as_ref()
            .expect("global store not initialized")
            .clone()
    })
}

pub struct AppStores {
    pub editor_interactions: Rc<RefCell<EditorInteractionsStore>>,
    pub theme: Rc<ThemeInteractionsStore>,
    pub file_interactions: Rc<RefCell<FileInteractionsStore>>,
    pub file_actions: Rc<RefCell<FileActionsStore>>,
    pub icons: Rc<IconsInteractionsStore>,
    pub files: Rc<RefCell<Vec<EntryRc>>>,
    pub hotkeys_interactions: Rc<RefCell<HotkeysInteractionsStore>>,
    pub global_store: Rc<RefCell<GlobalStore>>,
}

pub fn set_all_stores(stores: AppStores) {
    set_editor_interactions(stores.editor_interactions);
    set_theme(stores.theme);
    set_file_interactions(stores.file_interactions);
    set_file_actions(stores.file_actions);
    set_icons(stores.icons);
    set_files(stores.files);
}
