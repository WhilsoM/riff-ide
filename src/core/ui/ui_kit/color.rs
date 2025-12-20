//! Color utilities for UI components.
//!
//! Provides a convenient `Color` type that wraps `egui::Color32` with helper methods.
//!
//! # Examples
//!
//! ```rust,no_run
//! use crate::core::ui::ui_kit::color::Color;
//!
//! // Using static methods
//! let red = Color::rgb(255, 0, 0);
//! let blue = Color::rgba(0, 0, 255, 255);
//!
//! // Using constants
//! let white = Color::WHITE;
//! let black = Color::BLACK;
//!
//! // In style
//! Style::new()
//!     .background_color(Color::rgb(50, 50, 50))
//!     .border(Color::WHITE, 1.0);
//! ```

use eframe::egui;

/// A convenient wrapper around `egui::Color32` with helper methods and constants.
///
/// This type can be used anywhere `egui::Color32` is expected, as it implements `Into<egui::Color32>`.
///
/// # Examples
///
/// ```rust,no_run
/// use crate::core::ui::ui_kit::color::Color;
///
/// // Create colors
/// let red = Color::rgb(255, 0, 0);
/// let transparent_blue = Color::rgba(0, 0, 255, 128);
///
/// // Use predefined colors
/// let white = Color::WHITE;
/// let black = Color::BLACK;
///
/// // Convert to egui::Color32 (automatic)
/// let color32: egui::Color32 = Color::rgb(100, 100, 100).into();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color(pub egui::Color32);

impl Color {
    /// Create a color from RGB values (0-255).
    ///
    /// Example:
    /// ```rust,no_run
    /// Color::rgb(255, 0, 0)    // Red
    /// Color::rgb(0, 255, 0)    // Green
    /// Color::rgb(0, 0, 255)     // Blue
    /// Color::rgb(128, 128, 128) // Gray
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(egui::Color32::from_rgb(r, g, b))
    }

    /// Create a color from RGBA values (0-255).
    ///
    /// Example:
    /// ```rust,no_run
    /// Color::rgba(255, 0, 0, 255)    // Opaque red
    /// Color::rgba(255, 0, 0, 128)    // Semi-transparent red
    /// Color::rgba(0, 0, 0, 0)        // Fully transparent
    /// ```
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(egui::Color32::from_rgba_unmultiplied(r, g, b, a))
    }

    /// Create a color from RGB float values (0.0-1.0).
    ///
    /// Example:
    /// ```rust,no_run
    /// Color::rgb_f32(1.0, 0.0, 0.0)  // Red
    /// Color::rgb_f32(0.5, 0.5, 0.5)  // Gray
    /// ```
    pub fn rgb_f32(r: f32, g: f32, b: f32) -> Self {
        Self(egui::Color32::from_rgb(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
        ))
    }

    /// Create a color from RGBA float values (0.0-1.0).
    ///
    /// Example:
    /// ```rust,no_run
    /// Color::rgba_f32(1.0, 0.0, 0.0, 1.0)  // Opaque red
    /// Color::rgba_f32(1.0, 0.0, 0.0, 0.5)  // Semi-transparent red
    /// ```
    pub fn rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(egui::Color32::from_rgba_unmultiplied(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (a * 255.0) as u8,
        ))
    }

    /// Create a color from a hex string (e.g., "#FF0000" or "FF0000").
    ///
    /// Example:
    /// ```rust,no_run
    /// Color::hex("#FF0000")  // Red
    /// Color::hex("00FF00")   // Green (with or without #)
    /// Color::hex("#0000FF80") // Blue with alpha
    /// ```
    pub fn hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        let a = if hex.len() >= 8 {
            u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
        } else {
            255
        };
        Self::rgba(r, g, b, a)
    }

    /// Get the underlying `egui::Color32`.
    pub fn to_color32(self) -> egui::Color32 {
        self.0
    }

    // Common color constants
    /// White color: `rgb(255, 255, 255)`
    pub const WHITE: Color = Color(egui::Color32::WHITE);
    /// Black color: `rgb(0, 0, 0)`
    pub const BLACK: Color = Color(egui::Color32::BLACK);
    /// Red color: `rgb(255, 0, 0)`
    pub const RED: Color = Color(egui::Color32::from_rgb(255, 0, 0));
    /// Green color: `rgb(0, 255, 0)`
    pub const GREEN: Color = Color(egui::Color32::from_rgb(0, 255, 0));
    /// Blue color: `rgb(0, 0, 255)`
    pub const BLUE: Color = Color(egui::Color32::from_rgb(0, 0, 255));
    /// Yellow color: `rgb(255, 255, 0)`
    pub const YELLOW: Color = Color(egui::Color32::from_rgb(255, 255, 0));
    /// Cyan color: `rgb(0, 255, 255)`
    pub const CYAN: Color = Color(egui::Color32::from_rgb(0, 255, 255));
    /// Magenta color: `rgb(255, 0, 255)`
    pub const MAGENTA: Color = Color(egui::Color32::from_rgb(255, 0, 255));
    /// Gray color: `rgb(128, 128, 128)`
    pub const GRAY: Color = Color(egui::Color32::from_rgb(128, 128, 128));
    /// Light gray color: `rgb(192, 192, 192)`
    pub const LIGHT_GRAY: Color = Color(egui::Color32::from_rgb(192, 192, 192));
    /// Dark gray color: `rgb(64, 64, 64)`
    pub const DARK_GRAY: Color = Color(egui::Color32::from_rgb(64, 64, 64));
    /// Transparent color: `rgba(0, 0, 0, 0)`
    pub const TRANSPARENT: Color = Color(egui::Color32::TRANSPARENT);
}

impl From<Color> for egui::Color32 {
    fn from(color: Color) -> Self {
        color.0
    }
}

impl From<egui::Color32> for Color {
    fn from(color: egui::Color32) -> Self {
        Self(color)
    }
}

impl AsRef<egui::Color32> for Color {
    fn as_ref(&self) -> &egui::Color32 {
        &self.0
    }
}
