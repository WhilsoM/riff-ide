use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::models::Entry;
use crate::core::stores::icons::IconsInteractionsStore;
use crate::core::ui::ui_kit::{Button, Separator, Text, View};
use crate::core::ui::widgets::{render_side_panel, SidePanel};
use crate::modules::file::components::FileList;
use crate::modules::file::stores::file_actions::FileActionsStore;
use crate::modules::file::stores::file_interactions::FileInteractionsStore;
use crate::modules::file::stores::theme::ThemeInteractionsStore;
use crate::rsx;

pub struct FileExplorer {
    files: Rc<RefCell<Vec<Entry>>>,
    icons: Rc<IconsInteractionsStore>,
    interactions: Rc<RefCell<FileInteractionsStore>>,
    actions: Rc<RefCell<FileActionsStore>>,
    theme: Rc<ThemeInteractionsStore>,
}

impl FileExplorer {
    pub fn new(
        files: Rc<RefCell<Vec<Entry>>>,
        icons: Rc<IconsInteractionsStore>,
        interactions: Rc<RefCell<FileInteractionsStore>>,
        actions: Rc<RefCell<FileActionsStore>>,
        theme: Rc<ThemeInteractionsStore>,
    ) -> Self {
        Self {
            files,
            icons,
            interactions,
            actions,
            theme,
        }
    }

    pub fn render(&self, ctx: &egui::Context) {
        let refresh_handler = {
            let actions = Rc::clone(&self.actions);
            Rc::new(move || {
                actions.borrow_mut().refresh_files();
            })
        };

        let new_file_handler = {
            let actions = Rc::clone(&self.actions);
            Rc::new(move || {
                actions.borrow_mut().create_new_file();
            })
        };

        let inner_view = rsx! {
                View {
                    align: "start".to_string(),
                    justify: "start".to_string(),
                    style: Some(self.theme.bg_main_200_style()),
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
                        FileList(
                            self.files.clone(),
                            self.icons.clone(),
                            self.interactions.clone(),
                            self.theme.clone()
                        )
                    }
                }
        };

        let panel_view = rsx! {
            SidePanel("left_panel", true, inner_view)
        };

        render_side_panel(ctx, "left_panel", true, panel_view);
    }
}
