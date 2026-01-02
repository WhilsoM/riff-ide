use eframe::egui;

use crate::core::types::types::Element;

pub trait Component {
    fn render(&self, ui: &mut egui::Ui);
}

pub struct RenderContext<'a> {
    pub ui: &'a mut egui::Ui,
}

impl<'a> RenderContext<'a> {
    pub fn new(ui: &'a mut egui::Ui) -> Self {
        Self { ui }
    }
}

#[derive(Default)]
pub enum Children {
    #[default]
    None,
    Single(Element),
    Multiple(Vec<Element>),
}

impl Clone for Children {
    fn clone(&self) -> Self {
        match self {
            Children::None => Children::None,
            Children::Single(child) => Children::Single(child.clone()),
            Children::Multiple(children) => Children::Multiple(children.clone()),
        }
    }
}

impl Children {
    pub fn render(&self, ui: &mut egui::Ui) {
        match self {
            Children::None => {}
            Children::Single(child) => child.render(ui),
            Children::Multiple(children) => {
                for child in children {
                    child.render(ui);
                }
            }
        }
    }
}

impl From<Element> for Children {
    fn from(component: Element) -> Self {
        Children::Single(component)
    }
}

impl From<Vec<Element>> for Children {
    fn from(components: Vec<Element>) -> Self {
        Children::Multiple(components)
    }
}

pub struct ComponentWrapper<F>
where
    F: Fn(&mut egui::Ui),
{
    render_fn: F,
}

impl<F> ComponentWrapper<F>
where
    F: Fn(&mut egui::Ui),
{
    pub fn new(render_fn: F) -> Self {
        Self { render_fn }
    }
}

impl<F> Component for ComponentWrapper<F>
where
    F: Fn(&mut egui::Ui),
{
    fn render(&self, ui: &mut egui::Ui) {
        (self.render_fn)(ui);
    }
}

pub trait ComponentWithProps: Component {
    type Props: Default + Clone;

    fn new() -> Self
    where
        Self: Sized;

    fn new_with_props(props: Self::Props) -> Self
    where
        Self: Sized;
}
