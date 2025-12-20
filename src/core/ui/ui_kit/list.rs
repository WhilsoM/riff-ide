use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;
use std::rc::Rc;

pub struct List<T> {
    props: ListProps<T>,
}

pub struct ListProps<T> {
    pub data: Vec<T>,
    pub render_item: Option<Rc<dyn Fn(&T, usize) -> Rc<dyn Component>>>,
    pub key_fn: Option<Rc<dyn Fn(&T, usize) -> String>>,
    pub children: Children,
    pub style: Option<Rc<crate::core::ui::ui_kit::style::Style>>,
}

impl<T> Default for ListProps<T> {
    fn default() -> Self {
        Self {
            data: vec![],
            render_item: None,
            key_fn: None,
            children: Children::None,
            style: None,
        }
    }
}

impl<T: Clone> Clone for ListProps<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            render_item: self.render_item.clone(),
            key_fn: self.key_fn.clone(),
            children: self.children.clone(),
            style: self.style.clone(),
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            props: ListProps::default(),
        }
    }

    pub fn new_with_props(props: ListProps<T>) -> Self {
        Self { props }
    }
}

impl<T: Clone> ComponentWithProps for List<T> {
    type Props = ListProps<T>;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl<T> Component for List<T> {
    fn render(&self, ui: &mut egui::Ui) {
        if let Some(style) = &self.props.style {
            apply_style_to_list(ui, style);
        }

        if let Some(render_item) = &self.props.render_item {
            for (index, item) in self.props.data.iter().enumerate() {
                let component = render_item(item, index);
                component.render(ui);
            }
        } else {
            for (index, _item) in self.props.data.iter().enumerate() {
                if index == 0 {
                    self.props.children.render(ui);
                }
            }
        }
    }
}

fn apply_style_to_list(ui: &mut egui::Ui, style: &crate::core::ui::ui_kit::style::Style) {
    if let Some(padding) = style.padding {
        ui.add_space(padding.y);
    }

    if let Some(width) = style.width {
        ui.set_width(width);
    }
    if let Some(height) = style.height {
        ui.set_height(height);
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

