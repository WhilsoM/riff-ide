use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use crate::core::ui::ui_kit::style::Style;
use eframe::egui;
use std::rc::Rc;

pub struct SelectableLabel {
    props: SelectableLabelProps,
}

#[derive(Clone, Default)]
pub struct SelectableLabelProps {
    pub selected: bool,
    pub text: String,
    pub on_click: Option<Rc<dyn Fn()>>,
    pub hover_color: Option<egui::Color32>,
    pub text_color: Option<egui::Color32>,
    pub children: Children,
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
