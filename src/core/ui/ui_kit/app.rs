use crate::core::lib::rsx::component::Component;
use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static PANEL_REGISTRY: RefCell<PanelRegistry> = RefCell::new(PanelRegistry::new());
}

#[derive(Clone)]
pub enum PanelData {
    Top {
        id: String,
        children: Rc<dyn Component>,
    },
    Bottom {
        id: String,
        children: Rc<dyn Component>,
    },
    Left {
        id: String,
        resizable: bool,
        default_width: Option<f32>,
        children: Rc<dyn Component>,
    },
    Right {
        id: String,
        resizable: bool,
        default_width: Option<f32>,
        children: Rc<dyn Component>,
    },
    Central {
        children: Rc<dyn Component>,
    },
}

struct PanelRegistry {
    panels: Vec<PanelData>,
}

impl PanelRegistry {
    fn new() -> Self {
        Self { panels: Vec::new() }
    }

    fn clear(&mut self) {
        self.panels.clear();
    }
}

pub fn register_panel(panel: PanelData) {
    PANEL_REGISTRY.with(|registry| {
        registry.borrow_mut().panels.push(panel);
    });
}

pub fn render_app(ctx: &egui::Context, _app_component: Rc<dyn Component>) {
    PANEL_REGISTRY.with(|registry| {
        let mut reg = registry.borrow_mut();
        let panels = std::mem::take(&mut reg.panels);

        for panel in panels {
            match panel {
                PanelData::Top { id, children } => {
                    render_top_panel(ctx, &id, children);
                }
                PanelData::Left {
                    id,
                    resizable,
                    default_width,
                    children,
                } => {
                    render_left_panel(ctx, &id, resizable, default_width, children);
                }
                PanelData::Right {
                    id,
                    resizable,
                    default_width,
                    children,
                } => {
                    render_right_panel(ctx, &id, resizable, default_width, children);
                }
                PanelData::Bottom { id, children } => {
                    render_bottom_panel(ctx, &id, children);
                }
                PanelData::Central { children } => {
                    crate::core::ui::ui_kit::central_panel::render_central_panel(ctx, children);
                }
            }
        }
    });
}

fn render_top_panel(ctx: &egui::Context, id: &str, children: Rc<dyn Component>) {
    egui::TopBottomPanel::top(egui::Id::new(id))
        .frame(egui::Frame::new().fill(ctx.style().visuals.panel_fill))
        .show(ctx, |ui| {
            ui.set_width(ui.available_width());
            children.render(ui);
        });
}

fn render_bottom_panel(ctx: &egui::Context, id: &str, children: Rc<dyn Component>) {
    egui::TopBottomPanel::bottom(egui::Id::new(id))
        .frame(egui::Frame::new().fill(ctx.style().visuals.panel_fill))
        .show(ctx, |ui| {
            children.render(ui);
        });
}

fn render_left_panel(
    ctx: &egui::Context,
    id: &str,
    resizable: bool,
    default_width: Option<f32>,
    children: Rc<dyn Component>,
) {
    let mut panel = egui::SidePanel::left(egui::Id::new(id))
        .frame(egui::Frame::new().fill(ctx.style().visuals.panel_fill))
        .resizable(resizable);

    if let Some(width) = default_width {
        panel = panel.default_width(width);
    }

    panel.show(ctx, |ui| {
        children.render(ui);
    });
}

fn render_right_panel(
    ctx: &egui::Context,
    id: &str,
    resizable: bool,
    default_width: Option<f32>,
    children: Rc<dyn Component>,
) {
    let mut panel = egui::SidePanel::right(egui::Id::new(id))
        .frame(egui::Frame::new().fill(ctx.style().visuals.panel_fill))
        .resizable(resizable);

    if let Some(width) = default_width {
        panel = panel.default_width(width);
    }

    panel.show(ctx, |ui| {
        children.render(ui);
    });
}
