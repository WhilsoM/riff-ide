//! Centralized style application system.
//!
//! This module provides a unified way to apply Style properties to egui::Frame
//! and UI components, ensuring all components automatically support new style features.

use eframe::egui::{self, Align};

use super::style::Style;
use crate::core::ui::ui_kit::style::{FlexDirection, Justify, JustifyContent};

/// Applies style properties to an egui::Frame.
///
/// This function centralizes all frame styling logic, so when new style properties
/// are added to Style, they automatically work in all components that use this function.
///
/// Example:
/// ```rust,no_run
/// use crate::core::ui::ui_kit::style_applicator::apply_frame_styles;
/// use crate::core::ui::ui_kit::style::Style;
///
/// let style = Style::new().background_color(Color::RED).padding(10.0);
/// let mut frame = egui::Frame::new();
/// apply_frame_styles(&style, &mut frame);
/// ```
pub fn apply_frame_styles(style: &Style, frame: &mut egui::Frame) {
    // Background color
    if let Some(bg_color) = style.background_color {
        *frame = frame.fill(bg_color);
    }

    // Border
    if let (Some(border_color), Some(border_width)) = (style.border_color, style.border_width) {
        *frame = frame.stroke(egui::Stroke::new(border_width, border_color));
    }

    // Border radius
    if let Some(radius) = style.border_radius {
        *frame = frame.corner_radius(egui::CornerRadius::same(radius.min(255.0) as u8));
    }

    // Padding
    let has_individual_padding = style.padding_left.is_some()
        || style.padding_right.is_some()
        || style.padding_top.is_some()
        || style.padding_bottom.is_some();

    if has_individual_padding {
        let left = style.padding_left.unwrap_or(0.0).min(127.0) as i8;
        let right = style.padding_right.unwrap_or(0.0).min(127.0) as i8;
        let top = style.padding_top.unwrap_or(0.0).min(127.0) as i8;
        let bottom = style.padding_bottom.unwrap_or(0.0).min(127.0) as i8;
        *frame = frame.inner_margin(egui::Margin {
            left,
            right,
            top,
            bottom,
        });
    } else if let Some(padding) = style.padding {
        let padding_val = padding.x.max(padding.y).min(127.0) as i8;
        *frame = frame.inner_margin(egui::Margin::same(padding_val));
    }
}

/// Applies style constraints (width, height) to egui::Ui.
///
/// Example:
/// ```rust,no_run
/// use crate::core::ui::ui_kit::style_applicator::apply_ui_constraints;
///
/// apply_ui_constraints(ui, &style);
/// ```
pub fn apply_ui_constraints(ui: &mut egui::Ui, style: &Style) {
    if let Some(height) = style.height {
        ui.set_height(height);
    }
    if let Some(width) = style.width {
        ui.set_width(width);
    }
}

/// Applies spacing and gap styles to egui::Ui.
///
/// Example:
/// ```rust,no_run
/// use crate::core::ui::ui_kit::style_applicator::apply_spacing;
///
/// apply_spacing(ui, &style, Some(8.0));
/// ```
pub fn apply_spacing(ui: &mut egui::Ui, style: &Style, default_spacing: Option<f32>) {
    if let Some(spacing) = default_spacing {
        ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);
    }

    if let Some(gap) = style.gap {
        ui.spacing_mut().item_spacing = egui::vec2(gap, gap);
    }
}

/// Gets the flex direction from style, defaulting to TopDown.
///
/// Example:
/// ```rust,no_run
/// use crate::core::ui::ui_kit::style_applicator::get_flex_direction;
/// use crate::core::ui::ui_kit::style::Style;
///
/// let style = Style::new();
/// let direction = get_flex_direction(&style);
/// ```
pub fn get_flex_direction(style: &Style) -> egui::Direction {
    match style.flex_direction {
        Some(FlexDirection::Row) | Some(FlexDirection::RowReverse) => egui::Direction::LeftToRight,
        _ => egui::Direction::TopDown,
    }
}

/// Gets the justification from a string, defaulting to Center.
/// Returns egui::Justify for use with egui layouts.
///
/// Example:
/// ```rust,no_run
/// use crate::core::ui::ui_kit::style_applicator::get_align;
/// use crate::core::ui::ui_kit::style::Align;
///
/// let align = get_align("center");
/// ```
pub fn get_align(align_str: &str) -> Align {
    match align_str {
        "start" => Align::Min,
        "end" => Align::Max,
        "center" => Align::Center,
        _ => Align::Min,
    }
}

/// Gets the flex value from a string, defaulting to 0.
/// Returns i8 for use with egui layouts.
///
/// Example:
/// ```rust,no_run
/// use crate::core::ui::ui_kit::style_applicator::get_flex;
///
/// let flex = get_flex("1");
/// ```
pub fn get_flex(flex: &str) -> i8 {
    match flex {
        "1" => 1,
        "0" => 0,
        _ => 0,
    }
}

/// Gets the Justify enum from a string, defaulting to Start.
///
/// Example:
/// ```rust,no_run
/// use crate::core::ui::ui_kit::style_applicator::get_justify;
/// use crate::core::ui::ui_kit::style::Justify;
///
/// let justify = get_justify("space-between");
/// ```
pub fn get_justify(justify_str: &str) -> Justify {
    match justify_str {
        "start" => Justify::Start,
        "end" => Justify::End,
        "center" => Justify::Center,
        "space-between" => Justify::SpaceBetween,
        "space-around" => Justify::SpaceAround,
        "space-evenly" => Justify::SpaceEvenly,
        _ => Justify::Start,
    }
}

/// Converts our Justify enum to egui::Align and justify flag for use with Layout.
/// Returns (align, main_justify) tuple.
///
/// In egui, justify-content is implemented through main_align and main_justify flag.
/// - Start/End/Center use main_align with main_justify=false
/// - SpaceBetween/SpaceAround/SpaceEvenly use main_justify=true with appropriate align
///
/// Example:
/// ```rust,no_run
/// use crate::core::ui::ui_kit::style_applicator::justify_to_egui;
/// use crate::core::ui::ui_kit::style::Justify;
///
/// let (align, justify) = justify_to_egui(Justify::Center);
/// ```
pub fn justify_to_egui(justify: Justify) -> (egui::Align, bool) {
    match justify {
        Justify::Start => (egui::Align::Min, false),
        Justify::End => (egui::Align::Max, false),
        Justify::Center => (egui::Align::Center, false),
        Justify::SpaceBetween => (egui::Align::Min, true),
        Justify::SpaceAround => (egui::Align::Center, true),
        Justify::SpaceEvenly => (egui::Align::Center, true),
    }
}
