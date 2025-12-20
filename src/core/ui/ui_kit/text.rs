use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;

pub struct Text {
    props: TextProps,
}

/// Properties for the `Text` component.
///
/// Example usage in `rsx!`:
/// ```rust,no_run
/// rsx! {
///     Text {
///         content: "Hello, World!".to_string(),
///     }
/// }
/// ```
#[derive(Clone, Default)]
pub struct TextProps {
    /// The text content to display.
    ///
    /// Example:
    /// ```rust,no_run
    /// content: "Hello, World!".to_string()
    /// ```
    pub content: String,
    /// Child components (rarely used for Text).
    pub children: Children,
}

impl Text {
    pub fn new() -> Self {
        Self {
            props: TextProps::default(),
        }
    }

    pub fn new_with_props(props: TextProps) -> Self {
        Self { props }
    }
}

impl ComponentWithProps for Text {
    type Props = TextProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for Text {
    fn render(&self, ui: &mut egui::Ui) {
        if !self.props.content.is_empty() {
            ui.label(&self.props.content);
        }
        self.props.children.render(ui);
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}
