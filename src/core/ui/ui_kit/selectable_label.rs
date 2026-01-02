use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use crate::core::types::types::Handler;
use crate::core::ui::ui_kit::style::Style;
use eframe::egui;
use std::rc::Rc;

pub struct SelectableLabel {
    props: SelectableLabelProps,
}

/// Properties for the `SelectableLabel` component - a clickable label with selection state.
///
/// Example usage in `rsx!`:
/// ```rust,no_run
/// let handler = Rc::new(|| println!("Selected!"));
/// rsx! {
///     SelectableLabel {
///         selected: true,
///         text: "Option 1".to_string(),
///         on_click: Some(handler),
///         text_color: Some(egui::Color32::WHITE),
///         hover_color: Some(egui::Color32::from_rgb(50, 50, 50)),
///     }
/// }
/// ```
#[derive(Clone, Default)]
pub struct SelectableLabelProps {
    /// Whether the label is currently selected.
    ///
    /// Example:
    /// ```rust,no_run
    /// selected: true   // Label appears selected
    /// selected: false  // Label appears unselected
    /// ```
    pub selected: bool,
    /// The text to display in the label.
    ///
    /// Example:
    /// ```rust,no_run
    /// text: "File name".to_string()
    /// ```
    pub text: String,
    /// Optional click handler function.
    ///
    /// Example:
    /// ```rust,no_run
    /// let handler = Rc::new(|| {
    ///     println!("Label clicked!");
    /// });
    /// on_click: Some(handler)
    /// ```
    pub on_click: Option<Handler>,
    /// Optional background color when hovering over the label.
    ///
    /// Example:
    /// ```rust,no_run
    /// hover_color: Some(egui::Color32::from_rgb(50, 50, 50))
    /// ```
    pub hover_color: Option<egui::Color32>,
    /// Optional text color.
    ///
    /// Example:
    /// ```rust,no_run
    /// text_color: Some(egui::Color32::WHITE)
    /// text_color: Some(egui::Color32::from_rgb(200, 200, 200))
    /// ```
    pub text_color: Option<egui::Color32>,
    /// Child components (rarely used for SelectableLabel).
    pub children: Children,
    /// Optional style object for advanced styling.
    pub style: Option<Rc<Style>>,
}

impl SelectableLabel {
    pub fn new() -> Self {
        Self {
            props: SelectableLabelProps::default(),
        }
    }

    pub fn new_with_props(props: SelectableLabelProps) -> Self {
        Self { props }
    }
}

impl ComponentWithProps for SelectableLabel {
    type Props = SelectableLabelProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for SelectableLabel {
    fn render(&self, ui: &mut egui::Ui) {
        let mut text = egui::RichText::new(&self.props.text);

        if let Some(color) = self.props.text_color {
            text = text.color(color);
        }

        let response = ui.selectable_label(self.props.selected, text);

        if response.hovered() {
            if let Some(hover_color) = self.props.hover_color {
                ui.painter().rect_filled(response.rect, 0.0, hover_color);
            }
        }

        if response.clicked() {
            if let Some(on_click) = &self.props.on_click {
                on_click();
            }
        }

        self.props.children.render(ui);
    }
}

impl Default for SelectableLabel {
    fn default() -> Self {
        Self::new()
    }
}
