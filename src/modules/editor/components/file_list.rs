use crate::core::lib::rsx::component::Children;
use crate::core::types::types::Element;
use crate::core::ui::ui_kit::style::{Align, Justify};
use crate::core::ui::ui_kit::{Style, StyleSheet, View};
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

    let s = StyleSheet::new().with(
        "center",
        Style::new().justify(Justify::Start).align(Align::Start),
    );

    rsx! {
        View {
            style: s.get("center"),
            children: Children::Multiple(children),
        }
    }
}
