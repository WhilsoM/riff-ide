use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;

pub struct ScrollArea {
    props: ScrollAreaProps,
}

#[derive(Clone, Default)]
pub struct ScrollAreaProps {
    pub children: Children,
    pub auto_shrink: Option<(bool, bool)>,
}

impl ScrollArea {
    pub fn new() -> Self {
        Self {
            props: ScrollAreaProps::default(),
        }
    }

    pub fn new_with_props(props: ScrollAreaProps) -> Self {
        Self { props }
    }
}

impl ComponentWithProps for ScrollArea {
    type Props = ScrollAreaProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for ScrollArea {
    fn render(&self, ui: &mut egui::Ui) {
        let auto_shrink = self.props.auto_shrink.unwrap_or((false, false));
        egui::ScrollArea::vertical()
            .auto_shrink([auto_shrink.0, auto_shrink.1])
            .show(ui, |ui| {
                self.props.children.render(ui);
            });
    }
}

impl Default for ScrollArea {
    fn default() -> Self {
        Self::new()
    }
}
