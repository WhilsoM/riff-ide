use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps, ComponentWrapper};
use eframe::egui;
use std::rc::Rc;

pub struct LeftPanel {
    props: LeftPanelProps,
}

#[derive(Clone, Default)]
pub struct LeftPanelProps {
    /// Unique identifier for the panel.
    pub id: String,
    /// Whether the panel can be resized by the user.
    ///
    /// Example:
    /// ```rust,no_run
    /// resizable: true   // User can resize
    /// resizable: false  // Fixed size
    /// ```
    pub resizable: bool,
    /// Default width of the panel in pixels.
    ///
    /// Example:
    /// ```rust,no_run
    /// default_width: Some(250.0)  // 250px wide
    /// default_width: None         // Use egui default
    /// ```
    pub default_width: Option<f32>,
    /// Child components to render in the left panel.
    pub children: Children,
}

impl LeftPanel {
    pub fn new() -> Self {
        Self {
            props: LeftPanelProps::default(),
        }
    }

    pub fn new_with_props(props: LeftPanelProps) -> Self {
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
            crate::core::ui::ui_kit::app::PanelData::Left {
                id: props.id.clone(),
                resizable: props.resizable,
                default_width: props.default_width,
                children: children_for_registry,
            },
        );

        Self { props }
    }
}

impl ComponentWithProps for LeftPanel {
    type Props = LeftPanelProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for LeftPanel {
    fn render(&self, ui: &mut egui::Ui) {
        self.props.children.render(ui);
    }
}

impl Default for LeftPanel {
    fn default() -> Self {
        Self::new()
    }
}
