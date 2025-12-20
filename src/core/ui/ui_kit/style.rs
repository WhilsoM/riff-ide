//! Style system for UI components.
//!
//! This module provides a CSS-like styling system with `Style` struct and `StyleSheet` for managing styles.
//!
//! # Examples
//!
//! ## Basic Style Usage
//!
//! ```rust,no_run
//! use crate::core::ui::ui_kit::style::Style;
//! use crate::core::ui::ui_kit::color::Color;
//! use std::rc::Rc;
//!
//! let style = Rc::new(
//!     Style::new()
//!         .background_color(Color::rgb(50, 50, 50))
//!         .padding(10.0)
//!         .border_radius(5.0)
//! );
//!
//! // Use in rsx!
//! rsx! {
//!     View {
//!         style: Some(style),
//!         children: {
//!             Text { content: "Styled view".to_string() }
//!         }
//!     }
//! }
//! ```
//!
//! ## StyleSheet Usage
//!
//! ```rust,no_run
//! use crate::core::ui::ui_kit::style::{StyleSheet, Style, FlexDirection};
//! use crate::core::ui::ui_kit::color::Color;
//!
//! let stylesheet = StyleSheet::new()
//!     .with("card", Style::new()
//!         .background_color(Color::rgb(30, 30, 30))
//!         .padding(16.0)
//!         .border_radius(8.0))
//!     .with("button", Style::new()
//!         .background_color(Color::rgb(100, 100, 200))
//!         .padding(8.0));
//!
//! // Get style by name
//! let card_style = stylesheet.get("card");
//! ```

use eframe::egui;
use std::collections::HashMap;
use std::rc::Rc;

/// Style properties for UI components.
///
/// Supports CSS-like styling with builder pattern for easy composition.
///
/// Example:
/// ```rust,no_run
/// use crate::core::ui::ui_kit::style::Style;
/// use crate::core::ui::ui_kit::color::Color;
///
/// let style = Style::new()
///     .background_color(Color::rgb(50, 50, 50))
///     .padding(10.0)
///     .border(Color::WHITE, 1.0)
///     .border_radius(5.0)
///     .width(200.0)
///     .height(100.0);
/// ```
#[derive(Clone, Default)]
pub struct Style {
    /// Background color of the component.
    ///
    /// Example:
    /// ```rust,no_run
    /// use crate::core::ui::ui_kit::color::Color;
    ///
    /// .background_color(Color::rgb(50, 50, 50))
    /// .background_color(Color::BLACK)
    /// .background_color(Color::WHITE)
    /// ```
    pub background_color: Option<egui::Color32>,
    /// Border color.
    ///
    /// Example:
    /// ```rust,no_run
    /// use crate::core::ui::ui_kit::color::Color;
    ///
    /// .border(Color::WHITE, 1.0)  // Sets both color and width
    /// ```
    pub border_color: Option<egui::Color32>,
    pub color: Option<egui::Color32>,
    pub border_width: Option<f32>,
    /// Border radius for rounded corners.
    ///
    /// Example:
    /// ```rust,no_run
    /// .border_radius(5.0)   // 5px radius
    /// .border_radius(10.0)  // 10px radius
    /// ```
    pub border_radius: Option<f32>,
    /// Uniform padding (x, y).
    ///
    /// Example:
    /// ```rust,no_run
    /// .padding_xy(10.0, 5.0)  // 10px horizontal, 5px vertical
    /// .padding(10.0)           // 10px on all sides
    /// ```
    pub padding: Option<egui::Vec2>,
    /// Top padding.
    pub padding_top: Option<f32>,
    /// Bottom padding.
    pub padding_bottom: Option<f32>,
    /// Left padding.
    pub padding_left: Option<f32>,
    /// Right padding.
    pub padding_right: Option<f32>,
    /// Uniform margin (x, y).
    ///
    /// Example:
    /// ```rust,no_run
    /// .margin_xy(10.0, 5.0)  // 10px horizontal, 5px vertical
    /// .margin(10.0)            // 10px on all sides
    /// ```
    pub margin: Option<egui::Vec2>,
    /// Top margin.
    pub margin_top: Option<f32>,
    /// Bottom margin.
    pub margin_bottom: Option<f32>,
    /// Left margin.
    pub margin_left: Option<f32>,
    /// Right margin.
    pub margin_right: Option<f32>,
    /// Fixed width in pixels.
    ///
    /// Example:
    /// ```rust,no_run
    /// .width(200.0)  // 200px width
    /// ```
    pub width: Option<f32>,
    /// Fixed height in pixels.
    ///
    /// Example:
    /// ```rust,no_run
    /// .height(100.0)  // 100px height
    /// ```
    pub height: Option<f32>,
    /// Minimum width in pixels.
    pub min_width: Option<f32>,
    /// Minimum height in pixels.
    pub min_height: Option<f32>,
    /// Maximum width in pixels.
    pub max_width: Option<f32>,
    /// Maximum height in pixels.
    pub max_height: Option<f32>,
    /// Overflow behavior.
    ///
    /// Example:
    /// ```rust,no_run
    /// .overflow(Overflow::Hidden)   // Hide overflow
    /// .overflow(Overflow::Scroll)   // Show scrollbars
    /// ```
    pub overflow: Option<Overflow>,
    /// Display type.
    ///
    /// Example:
    /// ```rust,no_run
    /// .display(Display::Flex)  // Flex layout
    /// .display(Display::None)   // Hide element
    /// ```
    pub display: Option<Display>,
    /// Flex direction for layout.
    ///
    /// Example:
    /// ```rust,no_run
    /// .flex_direction(FlexDirection::Row)     // Horizontal layout
    /// .flex_direction(FlexDirection::Column)  // Vertical layout
    /// ```
    pub flex_direction: Option<FlexDirection>,
    /// Alignment of items in flex container.
    ///
    /// Example:
    /// ```rust,no_run
    /// .align_items(AlignItems::Center)  // Center align
    /// .align_items(AlignItems::Start)   // Start align
    /// ```
    pub align_items: Option<AlignItems>,
    /// Justification of content in flex container.
    ///
    /// Example:
    /// ```rust,no_run
    /// .justify_content(JustifyContent::SpaceBetween)  // Space between items
    /// .justify_content(JustifyContent::Center)         // Center items
    /// ```
    pub justify_content: Option<JustifyContent>,
    /// Flex grow factor.
    pub flex: Option<f32>,
    /// Gap between flex items.
    ///
    /// Example:
    /// ```rust,no_run
    /// .gap(8.0)   // 8px gap between items
    /// ```
    pub gap: Option<f32>,
}

/// Overflow behavior for content that exceeds container bounds.
///
/// Example:
/// ```rust,no_run
/// .overflow(Overflow::Visible)  // Content visible outside bounds
/// .overflow(Overflow::Hidden)   // Content clipped
/// .overflow(Overflow::Scroll)   // Show scrollbars
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Overflow {
    /// Content is visible outside container bounds.
    Visible,
    /// Content is clipped to container bounds.
    Hidden,
    /// Show scrollbars when content overflows.
    Scroll,
}

/// Display type for component visibility and layout.
///
/// Example:
/// ```rust,no_run
/// .display(Display::Flex)   // Use flex layout
/// .display(Display::None)   // Hide component
/// .display(Display::Block) // Block layout
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Display {
    /// Use flexbox layout.
    Flex,
    /// Hide the component.
    None,
    /// Use block layout.
    Block,
}

/// Flex direction for layout orientation.
///
/// Example:
/// ```rust,no_run
/// .flex_direction(FlexDirection::Row)         // Horizontal (left to right)
/// .flex_direction(FlexDirection::Column)      // Vertical (top to bottom)
/// .flex_direction(FlexDirection::RowReverse)  // Horizontal (right to left)
/// .flex_direction(FlexDirection::ColumnReverse) // Vertical (bottom to top)
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexDirection {
    /// Horizontal layout (left to right).
    Row,
    /// Vertical layout (top to bottom).
    Column,
    /// Horizontal layout (right to left).
    RowReverse,
    /// Vertical layout (bottom to top).
    ColumnReverse,
}

/// Alignment of items along the cross axis in flex container.
///
/// Example:
/// ```rust,no_run
/// .align_items(AlignItems::Start)   // Align to start
/// .align_items(AlignItems::Center)  // Center align
/// .align_items(AlignItems::End)     // Align to end
/// .align_items(AlignItems::Stretch) // Stretch to fill
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignItems {
    /// Align items to the start.
    Start,
    /// Align items to the end.
    End,
    /// Center align items.
    Center,
    /// Stretch items to fill container.
    Stretch,
}

/// Justification of content along the main axis in flex container.
///
/// Example:
/// ```rust,no_run
/// .justify_content(JustifyContent::Start)         // Align to start
/// .justify_content(JustifyContent::Center)        // Center items
/// .justify_content(JustifyContent::SpaceBetween)  // Space between items
/// .justify_content(JustifyContent::SpaceAround)    // Space around items
/// .justify_content(JustifyContent::SpaceEvenly)    // Equal space everywhere
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JustifyContent {
    /// Align to start.
    Start,
    /// Align to end.
    End,
    /// Center items.
    Center,
    /// Space between items (no space at edges).
    SpaceBetween,
    /// Space around items (equal space on both sides).
    SpaceAround,
    /// Equal space between all items and edges.
    SpaceEvenly,
}

impl Style {
    /// Create a new empty style.
    ///
    /// Example:
    /// ```rust,no_run
    /// let style = Style::new()
    ///     .background_color(egui::Color32::BLACK)
    ///     .padding(10.0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set background color.
    ///
    /// Example:
    /// ```rust,no_run
    /// .background_color(egui::Color32::from_rgb(50, 50, 50))
    /// ```
    /// Set background color.
    ///
    /// Accepts both `Color` and `egui::Color32`.
    ///
    /// Example:
    /// ```rust,no_run
    /// use crate::core::ui::ui_kit::color::Color;
    ///
    /// .background_color(Color::rgb(50, 50, 50))
    /// .background_color(Color::WHITE)
    /// .background_color(Color::hex("#FF0000"))
    /// ```
    pub fn background_color(mut self, color: impl Into<egui::Color32>) -> Self {
        self.background_color = Some(color.into());
        self
    }

    /// Set border color and width.
    ///
    /// Accepts both `Color` and `egui::Color32`.
    ///
    /// Example:
    /// ```rust,no_run
    /// use crate::core::ui::ui_kit::color::Color;
    ///
    /// .border(Color::WHITE, 1.0)  // White border, 1px width
    /// .border(Color::rgb(255, 0, 0), 2.0)  // Red border, 2px width
    /// ```
    pub fn border(mut self, color: impl Into<egui::Color32>, width: f32) -> Self {
        self.border_color = Some(color.into());
        self.border_width = Some(width);
        self
    }

    pub fn color(mut self, color: egui::Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Set uniform padding on all sides.
    ///
    /// Example:
    /// ```rust,no_run
    /// .padding(10.0)  // 10px on all sides
    /// ```
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(egui::Vec2::new(padding, padding));
        self
    }

    /// Set padding with different x and y values.
    ///
    /// Example:
    /// ```rust,no_run
    /// .padding_xy(10.0, 5.0)  // 10px horizontal, 5px vertical
    /// ```
    pub fn padding_xy(mut self, x: f32, y: f32) -> Self {
        self.padding = Some(egui::Vec2::new(x, y));
        self
    }

    /// Set horizontal padding (left and right).
    ///
    /// Example:
    /// ```rust,no_run
    /// .padding_horizontal(10.0)  // 10px left and right
    /// ```
    pub fn padding_horizontal(mut self, padding: f32) -> Self {
        self.padding_left = Some(padding);
        self.padding_right = Some(padding);
        self
    }

    /// Set vertical padding (top and bottom).
    ///
    /// Example:
    /// ```rust,no_run
    /// .padding_vertical(10.0)  // 10px top and bottom
    /// ```
    pub fn padding_vertical(mut self, padding: f32) -> Self {
        self.padding_top = Some(padding);
        self.padding_bottom = Some(padding);
        self
    }

    /// Set uniform margin on all sides.
    ///
    /// Example:
    /// ```rust,no_run
    /// .margin(10.0)  // 10px margin on all sides
    /// ```
    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = Some(egui::Vec2::new(margin, margin));
        self
    }

    /// Set margin with different x and y values.
    ///
    /// Example:
    /// ```rust,no_run
    /// .margin_xy(10.0, 5.0)  // 10px horizontal, 5px vertical
    /// ```
    pub fn margin_xy(mut self, x: f32, y: f32) -> Self {
        self.margin = Some(egui::Vec2::new(x, y));
        self
    }

    /// Set width.
    ///
    /// Example:
    /// ```rust,no_run
    /// .width(200.0)  // 200px width
    /// ```
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height.
    ///
    /// Example:
    /// ```rust,no_run
    /// .height(100.0)  // 100px height
    /// ```
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set both width and height.
    ///
    /// Example:
    /// ```rust,no_run
    /// .size(200.0, 100.0)  // 200px width, 100px height
    /// ```
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set overflow behavior.
    ///
    /// Example:
    /// ```rust,no_run
    /// .overflow(Overflow::Hidden)  // Hide overflow
    /// ```
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    /// Set display type.
    ///
    /// Example:
    /// ```rust,no_run
    /// .display(Display::Flex)  // Use flex layout
    /// ```
    pub fn display(mut self, display: Display) -> Self {
        self.display = Some(display);
        self
    }

    /// Set flex direction.
    ///
    /// Example:
    /// ```rust,no_run
    /// .flex_direction(FlexDirection::Row)  // Horizontal layout
    /// ```
    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.flex_direction = Some(direction);
        self
    }

    /// Set align items.
    ///
    /// Example:
    /// ```rust,no_run
    /// .align_items(AlignItems::Center)  // Center align
    /// ```
    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.align_items = Some(align);
        self
    }

    /// Set justify content.
    ///
    /// Example:
    /// ```rust,no_run
    /// .justify_content(JustifyContent::SpaceBetween)  // Space between items
    /// ```
    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.justify_content = Some(justify);
        self
    }

    /// Set gap between flex items.
    ///
    /// Example:
    /// ```rust,no_run
    /// .gap(8.0)  // 8px gap between items
    /// ```
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap);
        self
    }
}

/// StyleSheet for managing named styles (similar to CSS classes).
///
/// Allows you to define reusable styles and access them by name.
///
/// Example:
/// ```rust,no_run
/// use crate::core::ui::ui_kit::style::{StyleSheet, Style, FlexDirection};
/// use crate::core::ui::ui_kit::color::Color;
/// use std::rc::Rc;
///
/// let stylesheet = StyleSheet::new()
///     .with("card", Style::new()
///         .background_color(Color::rgb(30, 30, 30))
///         .padding(16.0)
///         .border_radius(8.0))
///     .with("button", Style::new()
///         .background_color(Color::rgb(100, 100, 200))
///         .padding(8.0));
///
/// // Use in rsx!
/// let card_style = stylesheet.get("card");
/// rsx! {
///     View {
///         style: card_style,
///         children: {
///             Text { content: "Card content".to_string() }
///         }
///     }
/// }
/// ```
pub struct StyleSheet {
    /// Default container style.
    pub container: Rc<Style>,
    /// Default button container style (row layout with gap and padding).
    pub button_container: Rc<Style>,
    /// Optional explorer panel style.
    pub explorer_panel: Option<Rc<Style>>,
    /// Optional editor panel style.
    pub editor_panel: Option<Rc<Style>>,
    /// Optional file item style.
    pub file_item: Option<Rc<Style>>,
    /// Optional file item hover style.
    pub file_item_hover: Option<Rc<Style>>,
    /// Optional primary text style.
    pub text_primary: Option<Rc<Style>>,
    /// Optional secondary text style.
    pub text_secondary: Option<Rc<Style>>,
    /// Custom named styles.
    custom_styles: HashMap<String, Rc<Style>>,
}

impl StyleSheet {
    /// Create a new StyleSheet with default styles.
    ///
    /// Example:
    /// ```rust,no_run
    /// let stylesheet = StyleSheet::new()
    ///     .with("myStyle", Style::new().padding(10.0));
    /// ```
    pub fn new() -> Self {
        Self {
            container: Rc::new(Style::new()),
            button_container: Rc::new(
                Style::new()
                    .flex_direction(FlexDirection::Row)
                    .gap(8.0)
                    .padding(8.0),
            ),
            explorer_panel: None,
            editor_panel: None,
            file_item: None,
            file_item_hover: None,
            text_primary: None,
            text_secondary: None,
            custom_styles: HashMap::new(),
        }
    }

    /// Create a custom StyleSheet using a closure.
    ///
    /// Example:
    /// ```rust,no_run
    /// let stylesheet = StyleSheet::custom(|| {
    ///     StyleSheet::new()
    ///         .with("custom", Style::new().padding(10.0))
    /// });
    /// ```
    pub fn custom<F>(f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        f()
    }

    /// Add a named style to the stylesheet.
    ///
    /// Example:
    /// ```rust,no_run
    /// let stylesheet = StyleSheet::new()
    ///     .with("card", Style::new()
    ///         .background_color(egui::Color32::from_rgb(30, 30, 30))
    ///         .padding(16.0))
    ///     .with("button", Style::new()
    ///         .background_color(egui::Color32::from_rgb(100, 100, 200)));
    /// ```
    pub fn with(mut self, name: &str, style: Style) -> Self {
        self.custom_styles.insert(name.to_string(), Rc::new(style));
        self
    }

    /// Get a style by name. Returns `None` if style doesn't exist.
    ///
    /// Supports both custom styles and built-in styles:
    /// - `"container"` - Default container style
    /// - `"button_container"` - Default button container style
    /// - `"explorer_panel"` - Explorer panel style (if set)
    /// - `"editor_panel"` - Editor panel style (if set)
    /// - `"file_item"` - File item style (if set)
    /// - `"file_item_hover"` - File item hover style (if set)
    /// - `"text_primary"` - Primary text style (if set)
    /// - `"text_secondary"` - Secondary text style (if set)
    /// - Custom style names added via `with()`
    ///
    /// Example:
    /// ```rust,no_run
    /// let stylesheet = StyleSheet::new()
    ///     .with("card", Style::new().padding(10.0));
    ///
    /// let card_style = stylesheet.get("card");  // Returns Some(Rc<Style>)
    /// let missing = stylesheet.get("missing");  // Returns None
    /// ```
    pub fn get(&self, name: &str) -> Option<Rc<Style>> {
        if let Some(style) = self.custom_styles.get(name) {
            return Some(style.clone());
        }

        match name {
            "container" => Some(self.container.clone()),
            "button_container" => Some(self.button_container.clone()),
            "explorer_panel" => self.explorer_panel.clone(),
            "editor_panel" => self.editor_panel.clone(),
            "file_item" => self.file_item.clone(),
            "file_item_hover" => self.file_item_hover.clone(),
            "text_primary" => self.text_primary.clone(),
            "text_secondary" => self.text_secondary.clone(),
            _ => None,
        }
    }
}

impl Default for StyleSheet {
    fn default() -> Self {
        Self::new()
    }
}
