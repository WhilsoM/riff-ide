use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;

pub struct ScrollArea {
    props: ScrollAreaProps,
}

/// Properties for the `ScrollArea` component - a scrollable container.
///
/// Example usage in `rsx!`:
/// ```rust,no_run
/// rsx! {
///     ScrollArea {
///         auto_shrink: Some((true, false)),
///         children: {
///             Text { content: "Long content...".to_string() };
///             Text { content: "More content...".to_string() }
///         }
///     }
/// }
/// ```
#[derive(Clone, Default)]
pub struct ScrollAreaProps {
    /// Child components to render inside the scrollable area.
    ///
    /// Example:
    /// ```rust,no_run
    /// children: {
    ///     Text { content: "Item 1".to_string() };
    ///     Text { content: "Item 2".to_string() }
    /// }
    /// ```
    pub children: Children,
    /// Optional auto-shrink behavior: `(horizontal, vertical)`.
    /// If `true`, the scroll area shrinks when content is smaller than available space.
    ///
    /// Example:
    /// ```rust,no_run
    /// auto_shrink: Some((false, true))   // Shrink vertically only
    /// auto_shrink: Some((true, true))    // Shrink both directions
    /// auto_shrink: None                  // Don't auto-shrink
    /// ```
    pub auto_shrink: Option<(bool, bool)>,
}

impl ScrollArea {
    pub fn new() -> Self {
        Self {
            props: ScrollAreaProps::default(),
        }
    }

    pub fn new_with_props(props: ScrollAreaProps) -> Self {
        Self { props }
    }
}

impl ComponentWithProps for ScrollArea {
    type Props = ScrollAreaProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for ScrollArea {
    fn render(&self, ui: &mut egui::Ui) {
        let auto_shrink = self.props.auto_shrink.unwrap_or((false, false));
        egui::ScrollArea::vertical()
            .auto_shrink([auto_shrink.0, auto_shrink.1])
            .show(ui, |ui| {
                self.props.children.render(ui);
            });
    }
}

impl Default for ScrollArea {
    fn default() -> Self {
        Self::new()
    }
}
