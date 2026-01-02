use std::rc::Rc;

use crate::core::{
    lib::rsx::component::{Children, Component, ComponentWithProps},
    types::types::Handler,
    ui::ui_kit::Style,
};
use eframe::egui;

pub struct Button {
    pub props: ButtonProps,
}

/// Properties for the `Button` component.
///
/// Example usage in `rsx!`:
/// ```rust,no_run
/// let handler = Rc::new(|| println!("Clicked!"));
/// rsx! {
///     Button {
///         text: "Click me".to_string(),
///         on_click: Some(handler),
///         enabled: true,
///     }
/// }
/// ```
pub struct ButtonProps {
    /// The text displayed on the button.
    ///
    /// Example:
    /// ```rust,no_run
    /// text: "Submit".to_string()
    /// ```
    pub text: String,
    /// Optional click handler function.
    ///
    /// Example:
    /// ```rust,no_run
    /// let handler = Rc::new(|| {
    ///     println!("Button clicked!");
    /// });
    /// on_click: Some(handler)
    /// ```
    pub on_click: Option<Handler>,
    /// Child components (rarely used for Button).
    pub children: Children,
    /// Whether the button is enabled (clickable).
    ///
    /// Example:
    /// ```rust,no_run
    /// enabled: true   // Button is clickable
    /// enabled: false  // Button is disabled
    /// ```
    pub enabled: bool,
    /// Optional button style (width, height, padding).
    ///
    /// Example:
    /// ```rust,no_run
    /// style: Some(ButtonStyle {
    ///     width: Some(200.0),
    ///     height: Some(40.0),
    ///     padding: Some(egui::Vec2::new(12.0, 6.0)),
    ///     ..Default::default()
    /// })
    /// ```
    pub style: Option<Rc<Style>>,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            text: String::new(),
            on_click: None,
            children: Children::None,
            enabled: true,
            style: None,
        }
    }
}

impl Clone for ButtonProps {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            on_click: self.on_click.clone(),
            children: self.children.clone(),
            enabled: self.enabled,
            style: self.style.clone(),
        }
    }
}

pub type Props = ButtonProps;

/// Style properties for the `Button` component.
///
/// Example usage:
/// ```rust,no_run
/// ButtonStyle {
///     width: Some(200.0),
///     height: Some(40.0),
///     min_width: Some(100.0),
///     min_height: Some(30.0),
///     padding: Some(egui::Vec2::new(12.0, 6.0)),
/// }
/// ```
#[derive(Clone, Default)]
pub struct ButtonStyle {
    /// Optional fixed width of the button.
    ///
    /// Example:
    /// ```rust,no_run
    /// width: Some(200.0)  // Fixed 200px width
    /// width: None         // Auto width
    /// ```
    pub width: Option<f32>,
    /// Optional fixed height of the button.
    ///
    /// Example:
    /// ```rust,no_run
    /// height: Some(40.0)  // Fixed 40px height
    /// ```
    pub height: Option<f32>,
    /// Optional minimum width of the button. Defaults to 100.0.
    ///
    /// Example:
    /// ```rust,no_run
    /// min_width: Some(100.0)  // Minimum 100px width
    /// ```
    pub min_width: Option<f32>,
    /// Optional minimum height of the button. Defaults to 30.0.
    ///
    /// Example:
    /// ```rust,no_run
    /// min_height: Some(30.0)  // Minimum 30px height
    /// ```
    pub min_height: Option<f32>,
    /// Optional padding inside the button. Defaults to (12.0, 6.0).
    ///
    /// Example:
    /// ```rust,no_run
    /// padding: Some(egui::Vec2::new(12.0, 6.0))  // 12px horizontal, 6px vertical
    /// ```
    pub padding: Option<egui::Vec2>,
    /// Optional style implemented from Style
    /// ```rust,no_run
    /// background_color: Some(Color::BLACK)
    /// ```
    pub style: Option<Rc<Style>>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            props: ButtonProps::default(),
        }
    }

    pub fn new_with_props(props: ButtonProps) -> Self {
        Self { props }
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        Self {
            props: ButtonProps {
                text: text.into(),
                ..Default::default()
            },
        }
    }

    pub fn with_text_and_handler(text: impl Into<String>, on_click: impl Fn() + 'static) -> Self {
        Self {
            props: ButtonProps {
                text: text.into(),
                on_click: Some(std::rc::Rc::new(on_click)),
                ..Default::default()
            },
        }
    }
}

impl ComponentWithProps for Button {
    type Props = ButtonProps;

    fn new() -> Self {
        Self {
            props: ButtonProps::default(),
        }
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self { props }
    }
}

impl Component for Button {
    fn render(&self, ui: &mut egui::Ui) {
        let button = egui::Button::new(&self.props.text);

        let mut response = if self.props.enabled {
            ui.add(button)
        } else {
            ui.add_enabled(false, button)
        };

        if let Some(style) = &self.props.style {
            if let Some(min_width) = style.min_width {
                if response.rect.width() < min_width {
                    response.rect.set_width(min_width);
                }
            }
            if let Some(min_height) = style.min_height {
                if response.rect.height() < min_height {
                    response.rect.set_height(min_height);
                }
            }
        }

        if response.clicked() {
            if let Some(ref handler) = self.props.on_click {
                handler();
            }
        }

        if !matches!(self.props.children, Children::None) {
            ui.allocate_ui_with_layout(
                response.rect.size(),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    self.props.children.render(ui);
                },
            );
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}
