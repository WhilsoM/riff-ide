use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use crate::core::ui::ui_kit::style::Style;
use crate::core::ui::ui_kit::style_applicator::{
    apply_spacing, apply_ui_constraints, get_align, get_flex_direction, get_justify,
    justify_to_egui,
};
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
        let style = &self.props.style;

        if let Some(style) = style {
            if let Some(margin) = style.margin {
                ui.add_space(margin.y);
            }
        }

        let mut frame = egui::Frame::new();

        if let Some(style) = style {
            crate::core::ui::ui_kit::style_applicator::apply_frame_styles(style, &mut frame);

            if style.padding.is_none()
                && style.padding_left.is_none()
                && style.padding_right.is_none()
                && style.padding_top.is_none()
                && style.padding_bottom.is_none()
            {
                if let Some(padding_val) = self.props.padding {
                    let padding_val = padding_val.min(127.0) as i8;
                    frame = frame.inner_margin(egui::Margin::same(padding_val));
                }
            }
        } else if let Some(padding_val) = self.props.padding {
            let padding_val = padding_val.min(127.0) as i8;
            frame = frame.inner_margin(egui::Margin::same(padding_val));
        }

        let render_content = |ui: &mut egui::Ui, _: Option<f32>| {
            let justify = get_justify(&self.props.justify);
            let (main_align, main_justify) = justify_to_egui(justify);

            let cross_align = get_align(&self.props.align);

            let direction = if let Some(style) = &self.props.style {
                get_flex_direction(style)
            } else {
                egui::Direction::TopDown
            };

            let mut layout = match direction {
                egui::Direction::LeftToRight => egui::Layout::left_to_right(main_align),
                egui::Direction::RightToLeft => egui::Layout::right_to_left(main_align),
                egui::Direction::TopDown => egui::Layout::top_down(main_align),
                egui::Direction::BottomUp => egui::Layout::bottom_up(main_align),
            };

            layout.cross_align = cross_align;

            if main_justify {
                layout.main_justify = true;
            }

            if matches!(justify, crate::core::ui::ui_kit::style::Justify::End)
                && matches!(direction, egui::Direction::LeftToRight)
            {
                layout.main_justify = true;
            }

            ui.with_layout(layout, |ui| {
                apply_spacing(
                    ui,
                    self.props.style.as_deref().unwrap_or(&Default::default()),
                    self.props.spacing,
                );

                self.props.children.render(ui);
            });
        };

        let width_constraint = style.as_ref().and_then(|s| s.width);
        let flex = style.as_ref().and_then(|s| s.flex).unwrap_or(0);

        let direction = if let Some(style) = &self.props.style {
            get_flex_direction(style)
        } else {
            egui::Direction::TopDown
        };

        if width_constraint.is_some() {
            frame.show(ui, |ui| {
                if let Some(style) = style {
                    apply_ui_constraints(ui, style);
                }
                render_content(ui, None);
            });
        } else if flex > 0 {
            let max_width = ui.max_rect().width();
            let available_width = ui.available_width().min(max_width);
            let has_justify_end = matches!(
                get_justify(&self.props.justify),
                crate::core::ui::ui_kit::style::Justify::End
            ) && matches!(direction, egui::Direction::LeftToRight);

            match direction {
                egui::Direction::LeftToRight => {
                    if has_justify_end {
                        let (rect, _response) = ui.allocate_exact_size(
                            egui::vec2(available_width, ui.available_height()),
                            egui::Sense::hover(),
                        );
                        frame.show(ui, |ui| {
                            ui.set_clip_rect(rect);
                            ui.set_max_width(available_width);
                            if let Some(style) = style {
                                apply_ui_constraints(ui, style);
                            }
                            render_content(ui, Some(available_width));
                        });
                    } else {
                        ui.horizontal(|ui| {
                            frame.show(ui, |ui| {
                                // Set max width to prevent overflow
                                ui.set_max_width(available_width);
                                // Set width to fill available space
                                ui.set_width(available_width);
                                if let Some(style) = style {
                                    apply_ui_constraints(ui, style);
                                }
                                render_content(ui, Some(available_width));
                            });
                        });
                    }
                }
                _ => {
                    ui.vertical(|ui| {
                        frame.show(ui, |ui| {
                            // Set max width to prevent overflow
                            ui.set_max_width(available_width);
                            ui.set_width(available_width);
                            if let Some(style) = style {
                                apply_ui_constraints(ui, style);
                            }
                            render_content(ui, Some(available_width));
                        });
                    });
                }
            }
        } else {
            match direction {
                egui::Direction::LeftToRight => {
                    ui.horizontal(|ui| {
                        frame.show(ui, |ui| {
                            if let Some(style) = style {
                                apply_ui_constraints(ui, style);
                            }
                            render_content(ui, None);
                        });
                    });
                }
                _ => {
                    ui.vertical(|ui| {
                        frame.show(ui, |ui| {
                            if let Some(style) = style {
                                apply_ui_constraints(ui, style);
                            }
                            render_content(ui, None);
                        });
                    });
                }
            }
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}
