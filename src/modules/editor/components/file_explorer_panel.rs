use crate::core::types::types::Element;
use crate::core::ui::ui_kit::{Button, Separator, Text, View};
use crate::modules::editor::components::{FileList, LeftPanel};
use crate::modules::editor::stores::file::file_actions::file_actions_store;
use crate::modules::editor::stores::theme_store;
use crate::{on_click, rsx};
use riff_rsx_macro::component;

#[component]
pub fn FileExplorerPanel(ctx: egui::Context) -> Element {
    let theme = theme_store();
    let ctx_refresh = ctx.clone();
    let ctx_new_file = ctx.clone();
    let ctx_file_list = ctx.clone();

    fn refresh_handler(ctx: egui::Context) {
        println!("Refreshing file explorer...");
        file_actions_store().refresh_files(&ctx);
    }

    fn new_file_handler(ctx: egui::Context) {
        println!("Creating new file...");
        file_actions_store().create_new_file(&ctx);
    }

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
                                    on_click: Some(on_click!(refresh_handler, ctx_refresh)),
                                };
                                Button {
                                    text: "New File".to_string(),
                                    on_click: Some(on_click!(new_file_handler, ctx_new_file)),
                                }
                            }
                        };
                        Separator {};
                        FileList(ctx_file_list)
                    }
                }
            }
        }
    }
}
