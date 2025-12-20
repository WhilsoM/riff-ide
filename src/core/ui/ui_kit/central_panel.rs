use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps, ComponentWrapper};
use eframe::egui;
use std::rc::Rc;

pub struct CentralPanel {
    props: CentralPanelProps,
}

/// Properties for the `CentralPanel` component - the main content area.
///
/// Example usage in `rsx!`:
/// ```rust,no_run
/// rsx! {
///     CentralPanel {
///         children: {
///             View {
///                 align: "center".to_string(),
///                 justify: "center".to_string(),
///                 children: {
///                     Text { content: "Main content".to_string() }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[derive(Clone, Default)]
pub struct CentralPanelProps {
    /// Child components to render in the central panel.
    ///
    /// Example:
    /// ```rust,no_run
    /// children: {
    ///     View {
    ///         align: "center".to_string(),
    ///         justify: "center".to_string(),
    ///         children: {
    ///             Text { content: "Content".to_string() }
    ///         }
    ///     }
    /// }
    /// ```
    pub children: Children,
}

impl CentralPanel {
    pub fn new() -> Self {
        Self {
            props: CentralPanelProps::default(),
        }
    }

    pub fn new_with_props(props: CentralPanelProps) -> Self {
        let children = match &props.children {
            Children::None => return Self { props },
            Children::Single(child) => child.clone(),
            Children::Multiple(children) => {
                if children.is_empty() {
                    return Self { props };
                }
                if children.len() == 1 {
                    children[0].clone()
                } else {
                    use crate::core::lib::rsx::component::ComponentWrapper;
                    let children_clone = children.clone();
                    Rc::new(ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                        for child in &children_clone {
                            child.render(ui);
                        }
                    })) as Rc<dyn Component>
                }
            }
        };

        crate::core::ui::ui_kit::app::register_panel(
            crate::core::ui::ui_kit::app::PanelData::Central { children },
        );

        Self { props }
    }
}

impl ComponentWithProps for CentralPanel {
    type Props = CentralPanelProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for CentralPanel {
    fn render(&self, ui: &mut egui::Ui) {
        self.props.children.render(ui);
    }
}

impl Default for CentralPanel {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_central_panel(ctx: &egui::Context, children: std::rc::Rc<dyn Component>) {
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(ctx.style().visuals.window_fill))
        .show(ctx, |ui| {
            children.render(ui);
        });
}
