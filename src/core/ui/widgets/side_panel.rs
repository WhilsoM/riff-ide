use crate::core::lib::rsx::component::Component;
use eframe::egui;
use std::rc::Rc;

pub fn SidePanel(_id: &str, _resizable: bool, children: Rc<dyn Component>) -> Rc<dyn Component> {
    children
}

pub fn render_side_panel(ctx: &egui::Context, id: &str, resizable: bool, children: Rc<dyn Component>) {
    egui::SidePanel::left(egui::Id::new(id))
        .resizable(resizable)
        .frame(egui::Frame::new().fill(ctx.style().visuals.panel_fill))
        .show(ctx, |ui| {
            children.render(ui);
        });
}
