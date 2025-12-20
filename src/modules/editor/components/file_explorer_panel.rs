use crate::core::lib::rsx::component::Element;
use crate::core::ui::ui_kit::{Button, Separator, Text, View};
use crate::modules::editor::components::{FileList, LeftPanel};
use crate::modules::editor::stores::file::file_actions::file_actions_store;
use crate::modules::editor::stores::theme_store;
use crate::rsx;
use riff_rsx_macro::component;

#[component]
pub fn FileExplorerPanel(ctx: eframe::egui::Context) -> Element {
    use std::rc::Rc;
    let theme = theme_store();

    let refresh_handler = {
        let ctx = ctx.clone();
        Rc::new(move || {
            file_actions_store().refresh_files(&ctx);
        })
    };

    let new_file_handler = {
        let ctx = ctx.clone();
        Rc::new(move || {
            file_actions_store().create_new_file(&ctx);
        })
    };

    rsx! {
        LeftPanel {
            id: "file_explorer".to_string(),
            resizable: true,
            default_width: Some(250.0),
            children: {
                View {
                    align: "start".to_string(),
                    justify: "start".to_string(),
                        style: Some(theme.bg_main_200_style(&ctx)),
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
            }
        }
    }
}
