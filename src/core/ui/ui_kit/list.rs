use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;
use std::rc::Rc;

pub struct List<T> {
    props: ListProps<T>,
}

/// Properties for the `List` component.
///
/// When using in `rsx!` macro, you can hover over field names to see their documentation.
pub struct ListProps<T> {
    /// Array of data of any type to display in the list
    pub data: Vec<T>,
    /// Function for rendering each element.
    /// Takes an element and its index, returns a component to display.
    /// Usually used with the `rsx!` macro.
    ///
    /// Example:
    /// ```rust,no_run
    /// let render_item = Rc::new(|item: &String, _index: usize| {
    ///     rsx! {
    ///         Text {
    ///             content: item.clone(),
    ///         }
    ///     }
    /// });
    /// ```
    pub render_item: Option<Rc<dyn Fn(&T, usize) -> Rc<dyn Component>>>,
    /// Optional function for generating a unique key for an element.
    /// Used for efficient list updates.
    ///
    /// Example:
    /// ```rust,no_run
    /// key_fn: Some(Rc::new(|item: &String, index: usize| {
    ///     format!("item-{}", index)
    /// }))
    /// ```
    pub key_fn: Option<Rc<dyn Fn(&T, usize) -> String>>,
    /// Children components (used when `render_item` is not provided)
    pub children: Children,
    /// Optional styles for the list (padding, width, height)
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
