use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use eframe::egui;

use crate::store;

#[derive(Debug, Clone)]
pub struct Tab {
    pub path: PathBuf,
    pub content: Rc<RefCell<String>>,
    pub original_content: String,
    pub is_dirty: bool,
}

impl PartialEq for Tab {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Tab {}

store! {
    pub struct EditorInteractionsStore {
        tabs: Vec<Tab> = vec![],
        active_tab_index: Option<usize> = None,
    }

    open_tab(&self, ctx: &egui::Context, path: PathBuf) {
        println!("[DEBUG] open_tab called with path: {:?}", path);
        let existing_index = self.tabs.borrow().iter().position(|t| t.path == path);

        if let Some(index) = existing_index {
            println!("[DEBUG] Tab already exists at index: {}", index);
            let mut reactive = self.reactive(ctx);
            *reactive.active_tab_index() = Some(index);
        } else {
            let tabs = self.tabs.borrow();
            let active_idx = self.active_tab_index.borrow();
            let should_create_new = if let Some(idx) = *active_idx {
                tabs.get(idx).map_or(false, |t| t.is_dirty)
            } else {
                false
            };

            drop(tabs);
            drop(active_idx);

            if should_create_new {
                println!("[DEBUG] Creating new tab (current tab is dirty)");
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let original_content = content.clone();
                    let new_tab = Tab {
                        path: path.clone(),
                        content: Rc::new(RefCell::new(content)),
                        original_content,
                        is_dirty: false,
                    };
                    let mut reactive = self.reactive(ctx);
                    let mut tabs = reactive.tabs();
                    let new_index = tabs.len();
                    tabs.push(new_tab);
                    *reactive.active_tab_index() = Some(new_index);
                    println!("[DEBUG] New tab created at index: {}, total tabs: {}", new_index, tabs.len());
                } else {
                    println!("[DEBUG] Failed to read file: {:?}", path);
                }
            } else {
                println!("[DEBUG] Replacing or creating tab (current tab not dirty)");
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let original_content = content.clone();
                    println!("[DEBUG] File read successfully, content length: {}", content.len());
                    let new_tab = Tab {
                        path: path.clone(),
                        content: Rc::new(RefCell::new(content)),
                        original_content,
                        is_dirty: false,
                    };
                    let mut reactive = self.reactive(ctx);
                    let mut tabs = reactive.tabs();
                    if let Some(active_idx) = *reactive.active_tab_index() {
                        if active_idx < tabs.len() {
                            tabs[active_idx] = new_tab;
                            println!("[DEBUG] Replaced tab at index: {}", active_idx);
                        } else {
                            tabs.push(new_tab);
                            *reactive.active_tab_index() = Some(tabs.len() - 1);
                            println!("[DEBUG] Pushed new tab, active index: {}", tabs.len() - 1);
                        }
                    } else {
                        tabs.push(new_tab);
                        *reactive.active_tab_index() = Some(tabs.len() - 1);
                        println!("[DEBUG] Pushed new tab (no active), active index: {}, total: {}", tabs.len() - 1, tabs.len());
                    }
                } else {
                    println!("[DEBUG] Failed to read file: {:?}", path);
                }
            }
        }
    }

    close_tab(&self, ctx: &egui::Context, index: usize) {
        let current_active = *self.active_tab_index.borrow();
        let mut reactive = self.reactive(ctx);
        let mut tabs = reactive.tabs();

        if index < tabs.len() {
            tabs.remove(index);

            if let Some(active_idx) = current_active {
                if active_idx == index {
                    if tabs.is_empty() {
                        *reactive.active_tab_index() = None;
                    } else if index > 0 {
                        *reactive.active_tab_index() = Some(index - 1);
                    } else {
                        *reactive.active_tab_index() = Some(0);
                    }
                } else if active_idx > index {
                    *reactive.active_tab_index() = Some(active_idx - 1);
                }
            }
        }
    }

    save_tab(&self, ctx: &egui::Context, index: usize) {
        let tab_path_content = {
            let tabs = self.tabs.borrow();
            tabs.get(index).map(|t| (t.path.clone(), t.content.borrow().clone()))
        };

        if let Some((path, content)) = tab_path_content {
            if let Err(e) = std::fs::write(&path, &content) {
                eprintln!("Failed to save file: {}", e);
            } else {
                let mut reactive = self.reactive(ctx);
                let mut tabs = reactive.tabs();
                if let Some(tab) = tabs.get_mut(index) {
                    tab.is_dirty = false;
                    tab.original_content = content;
                }
            }
        }
    }

    save_current_tab(&self, ctx: &egui::Context) {
        let active_idx = *self.active_tab_index.borrow();
        if let Some(idx) = active_idx {
            self.save_tab(ctx, idx);
        }
    }

    mark_tab_dirty(&self, ctx: &egui::Context, index: usize) {
        println!("[DEBUG] mark_tab_dirty called for index: {}", index);
        let mut reactive = self.reactive(ctx);
        let mut tabs = reactive.tabs();
        if let Some(tab) = tabs.get_mut(index) {
            println!("[DEBUG] Setting tab.is_dirty = true for index: {}", index);
            tab.is_dirty = true;
        } else {
            println!("[DEBUG] Tab at index {} not found!", index);
        }
    }

    mark_current_tab_dirty(&self, ctx: &egui::Context) {
        let active_idx = *self.active_tab_index.borrow();
        println!("[DEBUG] mark_current_tab_dirty called, active_idx: {:?}", active_idx);

        if let Some(idx) = active_idx {
            let (current_content, original_content) = {
                let tabs = self.tabs.borrow();
                if let Some(tab) = tabs.get(idx) {
                    let current = tab.content.borrow().clone();
                    let original = tab.original_content.clone();
                    println!("[DEBUG] Tab {} - current len: {}, original len: {}", idx, current.len(), original.len());
                    (Some(current), Some(original))
                } else {
                    println!("[DEBUG] Tab at index {} not found!", idx);
                    (None, None)
                }
            };

            if let (Some(content), Some(original)) = (current_content, original_content) {
                let is_different = content != original;
                println!("[DEBUG] Content comparison - is_different: {}", is_different);

                if is_different {
                    println!("[DEBUG] Content changed! Marking tab {} as dirty", idx);
                    self.mark_tab_dirty(ctx, idx);
                } else {
                    println!("[DEBUG] Content matches original, clearing dirty flag for tab {}", idx);
                    let mut reactive = self.reactive(ctx);
                    let mut tabs = reactive.tabs();
                    if let Some(tab) = tabs.get_mut(idx) {
                        tab.is_dirty = false;
                    }
                }
            } else {
                println!("[DEBUG] Failed to get content or original content");
            }
        } else {
            println!("[DEBUG] No active tab index");
        }
    }

    set_active_tab(&self, ctx: &egui::Context, index: usize) {
        let tabs_len = self.tabs.borrow().len();
        if index < tabs_len {
            let mut reactive = self.reactive(ctx);
            *reactive.active_tab_index() = Some(index);
        }
    }
}

impl EditorInteractionsStore {
    pub fn get_current_tab_path(&self, _ctx: &egui::Context) -> Option<PathBuf> {
        let active_idx = *self.active_tab_index.borrow();
        if let Some(idx) = active_idx {
            let tabs = self.tabs.borrow();
            tabs.get(idx).map(|t| t.path.clone())
        } else {
            None
        }
    }

    pub fn get_current_tab_text_ref(&self, _ctx: &egui::Context) -> Option<Rc<RefCell<String>>> {
        let active_idx = *self.active_tab_index.borrow();
        if let Some(idx) = active_idx {
            let tabs = self.tabs.borrow();
            tabs.get(idx).map(|t| t.content.clone())
        } else {
            None
        }
    }
}

pub fn editor_interactions_store() -> std::cell::Ref<'static, EditorInteractionsStore> {
    EditorInteractionsStore::instance()
}
