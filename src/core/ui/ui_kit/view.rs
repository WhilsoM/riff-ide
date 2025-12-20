use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use crate::core::ui::ui_kit::style::Style;
use eframe::egui;
use std::rc::Rc;

pub struct View {
    props: ViewProps,
}

#[derive(Clone, Default)]
pub struct ViewProps {
    pub align: String,
    pub justify: String,
    pub children: Children,
    pub padding: Option<f32>,
    pub spacing: Option<f32>,
    pub style: Option<Rc<Style>>,
}

impl View {
    pub fn new() -> Self {
        Self {
            props: ViewProps::default(),
        }
    }

    pub fn new_with_props(props: ViewProps) -> Self {
        Self { props }
    }
}

impl ComponentWithProps for View {
    type Props = ViewProps;

    fn new() -> Self {
        Self::new()
    }

    fn new_with_props(props: Self::Props) -> Self {
        Self::new_with_props(props)
    }
}

impl Component for View {
    fn render(&self, ui: &mut egui::Ui) {
        if let Some(style) = &self.props.style {
            apply_style_to_view(ui, style);
        }

        let align = match self.props.align.as_str() {
            "start" => egui::Align::Min,
            "end" => egui::Align::Max,
            "center" => egui::Align::Center,
            _ => egui::Align::Center,
        };

        if let Some(padding) = self.props.padding {
            ui.add_space(padding);
        }

        let direction = if let Some(style) = &self.props.style {
            match style.flex_direction {
                Some(crate::core::ui::ui_kit::style::FlexDirection::Row)
                | Some(crate::core::ui::ui_kit::style::FlexDirection::RowReverse) => {
                    egui::Direction::LeftToRight
                }
                _ => egui::Direction::TopDown,
            }
        } else {
            egui::Direction::TopDown
        };

        let layout = match direction {
            egui::Direction::LeftToRight => egui::Layout::left_to_right(align),
            egui::Direction::RightToLeft => egui::Layout::right_to_left(align),
            egui::Direction::TopDown => egui::Layout::top_down(align),
            egui::Direction::BottomUp => egui::Layout::bottom_up(align),
        };

        ui.with_layout(layout, |ui| {
            if let Some(spacing) = self.props.spacing {
                ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);
            }

            if let Some(style) = &self.props.style {
                if let Some(gap) = style.gap {
                    ui.spacing_mut().item_spacing = egui::vec2(gap, gap);
                }
            }

            self.props.children.render(ui);
        });
    }
}

fn apply_style_to_view(ui: &mut egui::Ui, style: &Style) {
    if let Some(margin) = style.margin {
        ui.add_space(margin.y);
    }

    if let Some(padding) = style.padding {
        ui.add_space(padding.y);
    }

    if let Some(width) = style.width {
        ui.set_width(width);
    }
    if let Some(height) = style.height {
        ui.set_height(height);
    }
    if let Some(min_width) = style.min_width {
        ui.set_min_width(min_width);
    }
    if let Some(min_height) = style.min_height {
        ui.set_min_height(min_height);
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}
