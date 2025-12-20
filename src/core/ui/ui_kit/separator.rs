use crate::core::lib::rsx::component::{Children, Component, ComponentWithProps};
use eframe::egui;

pub struct Separator {
    props: SeparatorProps,
}

/// Properties for the `Separator` component - a visual divider line.
///
/// Example usage in `rsx!`:
/// ```rust,no_run
/// rsx! {
///     View {
///         align: "start".to_string(),
///         justify: "start".to_string(),
///         children: {
///             Text { content: "Section 1".to_string() };
///             Separator {};
///             Text { content: "Section 2".to_string() }
///         }
///     }
/// }
/// ```
#[derive(Clone, Default)]
pub struct SeparatorProps {
    /// Child components (rarely used for Separator).
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
