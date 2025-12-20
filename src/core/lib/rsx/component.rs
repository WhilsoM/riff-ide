use eframe::egui;

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
    Single(Box<dyn Component>),
    Multiple(Vec<Box<dyn Component>>),
}

impl Clone for Children {
    fn clone(&self) -> Self {
        match self {
            Children::None => Children::None,
            Children::Single(_) => Children::None,
            Children::Multiple(_) => Children::None,
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
