use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TextEdit {
    props: TextEditProps,
}

/// Properties for the `TextEdit` component - a text input field.
///
/// Example usage in `rsx!`:
/// ```rust,no_run
/// let text = Rc::new(RefCell::new("Initial text".to_string()));
/// rsx! {
///     TextEdit {
///         value: text.clone(),
///         multiline: false,
///         font: Some("monospace".to_string()),
///     }
/// }
/// ```
#[derive(Clone, Default)]
pub struct TextEditProps {
    /// The text value wrapped in `Rc<RefCell<String>>` for mutable access.
    ///
    /// Example:
    /// ```rust,no_run
    /// let text = Rc::new(RefCell::new(String::new()));
    /// value: text.clone()
    /// ```
    pub value: Rc<RefCell<String>>,
    /// Whether the text edit is multiline (textarea) or single line.
    ///
    /// Example:
    /// ```rust,no_run
    /// multiline: true   // Multi-line text area
    /// multiline: false  // Single-line input
    /// ```
    pub multiline: bool,
    /// Optional font name. Currently supports "monospace".
    ///
    /// Example:
    /// ```rust,no_run
    /// font: Some("monospace".to_string())
    /// font: None  // Use default font
    /// ```
    pub font: Option<String>,
    /// Child components (rarely used for TextEdit).
    pub children: Children,
}

impl TextEdit {
    pub fn new() -> Self {
        Self {
            props: TextEditProps::default(),
        }
    }

    pub fn new_with_props(props: TextEditProps) -> Self {
        Self { props }
    }
}

impl ComponentWithProps for TextEdit {
    type Props = TextEditProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for TextEdit {
    fn render(&self, ui: &mut egui::Ui) {
        let mut text = self.props.value.borrow_mut();

        if self.props.multiline {
            let text_edit = if let Some(font_name) = &self.props.font {
                if font_name == "monospace" {
                    egui::TextEdit::multiline(&mut *text).font(egui::TextStyle::Monospace)
                } else {
                    egui::TextEdit::multiline(&mut *text)
                }
            } else {
                egui::TextEdit::multiline(&mut *text)
            };

            ui.add_sized([ui.available_width(), ui.available_height()], text_edit);
        } else {
            ui.text_edit_singleline(&mut *text);
        }

        self.props.children.render(ui);
    }
}

impl Default for TextEdit {
    fn default() -> Self {
        Self::new()
    }
}
