use crate::core::lib::rsx::component::ComponentWrapper;
use crate::core::types::types::Element;
use crate::modules::editor::components::{CodeEditorPanel, FileExplorerPanel, Navbar};
use riff_rsx_macro::component;
use std::rc::Rc;

#[component]
pub fn App(ctx: eframe::egui::Context) -> Element {
    let _navbar = Navbar(ctx.clone());
    let _explorer = FileExplorerPanel(ctx.clone());
    let _editor = CodeEditorPanel(ctx.clone());

    println!("[DEBUG] App: all components created");

    Rc::new(ComponentWrapper::new(|_ui: &mut eframe::egui::Ui| {})) as Element
}
