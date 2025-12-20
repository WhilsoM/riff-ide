use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;
use std::rc::Rc;

pub struct TopPanel {
    props: TopPanelProps,
}

#[derive(Clone, Default)]
pub struct TopPanelProps {
    /// Unique identifier for the panel.
    ///
    /// Example:
    /// ```rust,no_run
    /// id: "navbar".to_string()
    /// ```
    pub id: String,
    /// Child components to render in the top panel.
    ///
    /// Example:
    /// ```rust,no_run
    /// children: {
    ///     View {
    ///         align: "center".to_string(),
    ///         justify: "space-between".to_string(),
    ///         children: {
    ///             Text { content: "Content".to_string() }
    ///         }
    ///     }
    /// }
    /// ```
    pub children: Children,
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            props: TopPanelProps::default(),
        }
    }

    pub fn new_with_props(props: TopPanelProps) -> Self {
        let children_for_registry = match &props.children {
            Children::None => return Self { props },
            Children::Single(child) => child.clone(),
            Children::Multiple(children) => {
                if children.is_empty() {
                    return Self { props };
                }
                use crate::core::lib::rsx::component::ComponentWrapper;
                let children_clone = children.clone();
                Rc::new(ComponentWrapper::new(move |ui: &mut egui::Ui| {
                    for child in &children_clone {
                        child.render(ui);
                    }
                })) as Rc<dyn Component>
            }
        };

        crate::core::ui::ui_kit::app::register_panel(
            crate::core::ui::ui_kit::app::PanelData::Top {
                id: props.id.clone(),
                children: children_for_registry,
            },
        );

        Self { props }
    }
}

impl ComponentWithProps for TopPanel {
    type Props = TopPanelProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for TopPanel {
    fn render(&self, ui: &mut egui::Ui) {
        self.props.children.render(ui);
    }
}

impl Default for TopPanel {
    fn default() -> Self {
        Self::new()
    }
}
