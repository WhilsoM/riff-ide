use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TextEdit {
    props: TextEditProps,
}

#[derive(Clone, Default)]
pub struct TextEditProps {
    pub value: Rc<RefCell<String>>,
    pub multiline: bool,
    pub font: Option<String>,
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
