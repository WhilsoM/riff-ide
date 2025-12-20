use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;

pub struct Image {
    props: ImageProps,
}

/// Properties for the `Image` component.
///
/// Example usage in `rsx!`:
/// ```rust,no_run
/// let texture_id = load_texture(ctx);
/// rsx! {
///     Image {
///         texture_id: Some(texture_id),
///         size: Some(egui::Vec2::new(64.0, 64.0)),
///     }
/// }
/// ```
#[derive(Clone, Default)]
pub struct ImageProps {
    /// The texture ID to display. Required for the image to be visible.
    ///
    /// Example:
    /// ```rust,no_run
    /// texture_id: Some(icon_texture_id)
    /// ```
    pub texture_id: Option<egui::TextureId>,
    /// Optional size of the image. Defaults to 16x16 if not specified.
    ///
    /// Example:
    /// ```rust,no_run
    /// size: Some(egui::Vec2::new(32.0, 32.0))  // 32x32 pixels
    /// size: Some(egui::Vec2::new(64.0, 64.0))  // 64x64 pixels
    /// ```
    pub size: Option<egui::Vec2>,
    /// Child components (rarely used for Image).
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
