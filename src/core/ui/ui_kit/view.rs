use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use crate::core::ui::ui_kit::style::Style;
use eframe::egui;
use std::rc::Rc;

/// Properties for the `View` component - a flexible container.
///
/// Example usage in `rsx!`:
/// ```rust,no_run
/// rsx! {
///     View {
///         align: "start".to_string(),
///         justify: "center".to_string(),
///         padding: Some(10.0),
///         spacing: Some(8.0),
///         children: {
///             Text {
///                 content: "Content".to_string(),
///             }
///         }
///     }
/// }
/// ```
pub struct View {
    props: ViewProps,
}

#[derive(Clone, Default)]
pub struct ViewProps {
    /// Alignment of children: "start", "center", or "end".
    ///
    /// Example:
    /// ```rust,no_run
    /// align: "start".to_string()  // Align to start
    /// align: "center".to_string() // Center align
    /// align: "end".to_string()    // Align to end
    /// ```
    pub align: String,
    /// Justification of children: "start", "center", or "end".
    ///
    /// Example:
    /// ```rust,no_run
    /// justify: "start".to_string()
    /// ```
    pub justify: String,
    /// Child components to render inside the view.
    ///
    /// Example:
    /// ```rust,no_run
    /// children: {
    ///     Text { content: "Item 1".to_string() };
    ///     Text { content: "Item 2".to_string() }
    /// }
    /// ```
    pub children: Children,
    /// Optional padding value (applied uniformly).
    ///
    /// Example:
    /// ```rust,no_run
    /// padding: Some(10.0)  // 10px padding on all sides
    /// ```
    pub padding: Option<f32>,
    /// Optional spacing between children.
    ///
    /// Example:
    /// ```rust,no_run
    /// spacing: Some(8.0)  // 8px spacing between items
    /// ```
    pub spacing: Option<f32>,
    /// Optional style object for advanced styling (background, borders, etc.).
    ///
    /// Example:
    /// ```rust,no_run
    /// style: Some(Rc::new(Style {
    ///     background_color: Some(egui::Color32::from_rgb(255, 0, 0)),
    ///     padding: Some(egui::vec2(10.0, 10.0)),
    ///     ..Default::default()
    /// }))
    /// ```
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
            if let Some(margin) = style.margin {
                ui.add_space(margin.y);
            }
        }

        let mut frame = egui::Frame::new();

        if let Some(style) = &self.props.style {
            if let Some(bg_color) = style.background_color {
                frame = frame.fill(bg_color);
            }

            if let (Some(border_color), Some(border_width)) =
                (style.border_color, style.border_width)
            {
                frame = frame.stroke(egui::Stroke::new(border_width, border_color));
            }

            if let Some(radius) = style.border_radius {
                frame = frame.corner_radius(egui::CornerRadius::same(radius.min(255.0) as u8));
            }

            let has_individual_padding = style.padding_left.is_some()
                || style.padding_right.is_some()
                || style.padding_top.is_some()
                || style.padding_bottom.is_some();

            if has_individual_padding {
                let left = style.padding_left.unwrap_or(0.0).min(127.0) as i8;
                let right = style.padding_right.unwrap_or(0.0).min(127.0) as i8;
                let top = style.padding_top.unwrap_or(0.0).min(127.0) as i8;
                let bottom = style.padding_bottom.unwrap_or(0.0).min(127.0) as i8;
                frame = frame.inner_margin(egui::Margin {
                    left,
                    right,
                    top,
                    bottom,
                });
            } else if let Some(padding) = style.padding {
                let padding_val = padding.x.max(padding.y).min(127.0) as i8;
                frame = frame.inner_margin(egui::Margin::same(padding_val));
            } else if let Some(padding_val) = self.props.padding {
                let padding_val = padding_val.min(127.0) as i8;
                frame = frame.inner_margin(egui::Margin::same(padding_val));
            }
        } else if let Some(padding_val) = self.props.padding {
            let padding_val = padding_val.min(127.0) as i8;
            frame = frame.inner_margin(egui::Margin::same(padding_val));
        }

        frame.show(ui, |ui| {
            let align = match self.props.align.as_str() {
                "start" => egui::Align::Min,
                "end" => egui::Align::Max,
                "center" => egui::Align::Center,
                _ => egui::Align::Center,
            };

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
        });
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}
