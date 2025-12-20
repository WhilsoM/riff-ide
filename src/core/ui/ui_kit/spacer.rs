use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;

pub struct Spacer {
    props: SpacerProps,
}

/// Properties for the `Spacer` component - adds empty space.
///
/// Example usage in `rsx!`:
/// ```rust,no_run
/// rsx! {
///     View {
///         align: "start".to_string(),
///         justify: "start".to_string(),
///         children: {
///             Text { content: "Left".to_string() };
///             Spacer { size: 20.0 };
///             Text { content: "Right".to_string() }
///         }
///     }
/// }
/// ```
#[derive(Clone, Default)]
pub struct SpacerProps {
    /// The size of the spacer in pixels.
    ///
    /// Example:
    /// ```rust,no_run
    /// size: 10.0  // 10px spacing
    /// size: 20.0  // 20px spacing
    /// size: 50.0  // 50px spacing
    /// ```
    pub size: f32,
    /// Child components (rarely used for Spacer).
    pub children: Children,
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            props: SpacerProps::default(),
        }
    }

    pub fn new_with_props(props: SpacerProps) -> Self {
        Self { props }
    }
}

impl ComponentWithProps for Spacer {
    type Props = SpacerProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for Spacer {
    fn render(&self, ui: &mut egui::Ui) {
        ui.add_space(self.props.size);
        self.props.children.render(ui);
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}
