use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::models::Entry;
use crate::core::stores::icons::IconsInteractionsStore;
use crate::core::ui::ui_kit::{Button, Separator, Text, View};
use crate::core::ui::widgets::{SidePanel, render_side_panel};
use crate::modules::editor::components::FileList;
use crate::modules::editor::stores::context::get_theme;
use crate::modules::editor::stores::{
    EditorStores, FileActionsStore, FileInteractionsStore, theme_store,
};
use crate::rsx;

pub struct FileExplorer {
    files: Rc<RefCell<Vec<Entry>>>,
    stores: Rc<EditorStores>,
}

impl FileExplorer {
    pub fn new(
        files: Rc<RefCell<Vec<Entry>>>,
        icons: Rc<IconsInteractionsStore>,
        interactions: Rc<RefCell<FileInteractionsStore>>,
        actions: Rc<RefCell<FileActionsStore>>,
    ) -> Self {
        let stores = Rc::new(EditorStores::new(icons, interactions, actions, get_theme()));
        Self { files, stores }
    }

    pub fn render(&self, ctx: &egui::Context) {
        let refresh_handler = {
            let stores = self.stores.clone();
            let ctx = ctx.clone();
            Rc::new(move || {
                stores.file_actions.borrow().refresh_files(&ctx);
            })
        };

        let new_file_handler = {
            let stores = self.stores.clone();
            let ctx = ctx.clone();
            Rc::new(move || {
                stores.file_actions.borrow().create_new_file(&ctx);
            })
        };

        let inner_view = rsx! {
                View {
                    align: "start".to_string(),
                    justify: "start".to_string(),
                    style: Some(theme_store().bg_main_200_style(ctx)),
                    children: {
                        Text {
                            content: "Explorer".to_string(),
                        };
                        Separator {};
                        View {
                            align: "start".to_string(),
                            justify: "start".to_string(),
                            children: {
                                Button {
                                    text: "Refresh".to_string(),
                                    on_click: Some(refresh_handler),
                                };
                                Button {
                                    text: "New File".to_string(),
                                    on_click: Some(new_file_handler),
                                }
                            }
                        };
                        Separator {};
                        FileList(ctx.clone())
                    }
                }
        };

        let panel_view = rsx! {
            SidePanel("left_panel", true, inner_view)
        };

        render_side_panel(ctx, "left_panel", true, panel_view);
    }
}
