use eframe::egui;
use crate::core::lib::rsx::component::{Component, ComponentWithProps, Children};

pub struct Separator {
    props: SeparatorProps,
}

#[derive(Clone, Default)]
pub struct SeparatorProps {
    pub children: Children,
}

impl Separator {
    pub fn new() -> Self {
        Self {
            props: SeparatorProps::default(),
        }
    }

    pub fn new_with_props(props: SeparatorProps) -> Self {
        Self { props }
    }
}

impl ComponentWithProps for Separator {
    type Props = SeparatorProps;
    
    fn new() -> Self {
        Self::new()
    }
    
    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for Separator {
    fn render(&self, ui: &mut egui::Ui) {
        ui.separator();
        self.props.children.render(ui);
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

