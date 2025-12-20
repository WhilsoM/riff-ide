use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;

pub struct Button {
    pub props: ButtonProps,
}

pub struct ButtonProps {
    pub text: String,
    pub on_click: Option<std::rc::Rc<dyn Fn()>>,
    pub children: Children,
    pub enabled: bool,
    pub style: Option<ButtonStyle>,
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

#[derive(Clone)]
pub struct ButtonStyle {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub padding: Option<egui::Vec2>,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            min_width: Some(100.0),
            min_height: Some(30.0),
            padding: Some(egui::Vec2::new(12.0, 6.0)),
        }
    }
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
