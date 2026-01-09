use crate::core::lib::rsx::component::Children;
use crate::core::types::types::Element;
use crate::core::ui::ui_kit::View;
use crate::modules::editor::components::file_tree_item::FileTreeItem;
use crate::modules::editor::stores::context::get_files;
use crate::rsx;
use riff_rsx_macro::component;

#[component]
pub fn FileList(ctx: eframe::egui::Context) -> Element {
    let files = get_files();
    let entries = files.borrow();

    let children: Vec<Element> = entries
        .iter()
        .map(|entry_rc| FileTreeItem(entry_rc.clone(), 0, ctx.clone()))
        .collect();

    rsx! {
        View {
            align: "start".to_string(),
            justify: "start".to_string(),
            children: Children::Multiple(children),
        }
    }
}
