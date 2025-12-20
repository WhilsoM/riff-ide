use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;

pub struct Spacer {
    props: SpacerProps,
}

#[derive(Clone, Default)]
pub struct SpacerProps {
    pub size: f32,
    pub children: Children,
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            props: SpacerProps::default(),
        }
    }

    pub fn new_with_props(props: SpacerProps) -> Self {
        Self { props }
    }
}

impl ComponentWithProps for Spacer {
    type Props = SpacerProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for Spacer {
    fn render(&self, ui: &mut egui::Ui) {
        ui.add_space(self.props.size);
        self.props.children.render(ui);
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}
