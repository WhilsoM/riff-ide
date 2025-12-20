use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;

pub struct Image {
    props: ImageProps,
}

#[derive(Clone, Default)]
pub struct ImageProps {
    pub texture_id: Option<egui::TextureId>,
    pub size: Option<egui::Vec2>,
    pub children: Children,
}

impl Image {
    pub fn new() -> Self {
        Self {
            props: ImageProps::default(),
        }
    }

    pub fn new_with_props(props: ImageProps) -> Self {
        Self { props }
    }
}

impl ComponentWithProps for Image {
    type Props = ImageProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for Image {
    fn render(&self, ui: &mut egui::Ui) {
        if let Some(texture_id) = self.props.texture_id {
            if let Some(size) = self.props.size {
                ui.image((texture_id, size));
            } else {
                ui.image((texture_id, egui::Vec2::new(16.0, 16.0)));
            }
        }
        self.props.children.render(ui);
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}
