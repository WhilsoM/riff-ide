use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps, ComponentWrapper};
use eframe::egui;
use std::rc::Rc;

pub struct BottomPanel {
    props: BottomPanelProps,
}

#[derive(Clone, Default)]
pub struct BottomPanelProps {
    /// Unique identifier for the panel.
    pub id: String,
    /// Child components to render in the bottom panel.
    pub children: Children,
}

impl BottomPanel {
    pub fn new() -> Self {
        Self {
            props: BottomPanelProps::default(),
        }
    }

    pub fn new_with_props(props: BottomPanelProps) -> Self {
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
            crate::core::ui::ui_kit::app::PanelData::Bottom {
                id: props.id.clone(),
                children: children_for_registry,
            },
        );

        Self { props }
    }
}

impl ComponentWithProps for BottomPanel {
    type Props = BottomPanelProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for BottomPanel {
    fn render(&self, ui: &mut egui::Ui) {
        self.props.children.render(ui);
    }
}

impl Default for BottomPanel {
    fn default() -> Self {
        Self::new()
    }
}
