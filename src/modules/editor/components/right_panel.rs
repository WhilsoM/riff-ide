use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps, ComponentWrapper};
use eframe::egui;
use std::rc::Rc;

pub struct RightPanel {
    props: RightPanelProps,
}

#[derive(Clone, Default)]
pub struct RightPanelProps {
    /// Unique identifier for the panel.
    pub id: String,
    /// Whether the panel can be resized by the user.
    pub resizable: bool,
    /// Default width of the panel in pixels.
    pub default_width: Option<f32>,
    /// Child components to render in the right panel.
    pub children: Children,
}

impl RightPanel {
    pub fn new() -> Self {
        Self {
            props: RightPanelProps::default(),
        }
    }

    pub fn new_with_props(props: RightPanelProps) -> Self {
        let children_for_registry = match &props.children {
            Children::None => return Self { props },
            Children::Single(child) => child.clone(),
            Children::Multiple(children) => {
                if children.is_empty() {
                    return Self { props };
                }
                let children_clone = children.clone();
                Rc::new(ComponentWrapper::new(move |ui: &mut egui::Ui| {
                    for child in &children_clone {
                        child.render(ui);
                    }
                })) as Rc<dyn Component>
            }
        };

        crate::core::ui::ui_kit::app::register_panel(
            crate::core::ui::ui_kit::app::PanelData::Right {
                id: props.id.clone(),
                resizable: props.resizable,
                default_width: props.default_width,
                children: children_for_registry,
            },
        );

        Self { props }
    }
}

impl ComponentWithProps for RightPanel {
    type Props = RightPanelProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for RightPanel {
    fn render(&self, ui: &mut egui::Ui) {
        self.props.children.render(ui);
    }
}

impl Default for RightPanel {
    fn default() -> Self {
        Self::new()
    }
}
