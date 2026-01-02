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
use crate::{on_click, rsx};

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
        let stores_refresh_handler = self.stores.clone();
        let stores_new_file_handler = self.stores.clone();
        let ctx_refresh_handler = ctx.clone();
        let ctx_new_file_handler = ctx.clone();

        fn refresh_handler(stores: Rc<EditorStores>, ctx: egui::Context) {
            stores.file_actions.borrow().refresh_files(&ctx);
        }

        fn new_file_handler(stores: Rc<EditorStores>, ctx: egui::Context) {
            stores.file_actions.borrow().create_new_file(&ctx);
        }

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
                                    on_click: Some(on_click!(refresh_handler, stores_refresh_handler, ctx_refresh_handler)),
                                };
                                Button {
                                    text: "New File".to_string(),
                                    on_click: Some(on_click!(new_file_handler, stores_new_file_handler, ctx_new_file_handler)),
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
