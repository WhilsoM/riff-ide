use eframe::egui;
use crate::core::lib::rsx::component::Component;

pub fn SidePanel(
    _id: &str,
    _resizable: bool,
    children: impl Component,
) -> impl Component {
    children
}

pub fn render_side_panel(
    ctx: &egui::Context,
    id: &str,
    resizable: bool,
    children: impl Component,
) {
    egui::SidePanel::left(egui::Id::new(id))
        .resizable(resizable)
        .show(ctx, |ui| {
            children.render(ui);
        });
}

