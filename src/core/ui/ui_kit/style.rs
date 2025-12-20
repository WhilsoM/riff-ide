use eframe::egui;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct Style {
    pub background_color: Option<egui::Color32>,
    pub border_color: Option<egui::Color32>,
    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,
    pub padding: Option<egui::Vec2>,
    pub padding_top: Option<f32>,
    pub padding_bottom: Option<f32>,
    pub padding_left: Option<f32>,
    pub padding_right: Option<f32>,
    pub margin: Option<egui::Vec2>,
    pub margin_top: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub margin_left: Option<f32>,
    pub margin_right: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub overflow: Option<Overflow>,
    pub display: Option<Display>,
    pub flex_direction: Option<FlexDirection>,
    pub align_items: Option<AlignItems>,
    pub justify_content: Option<JustifyContent>,
    pub flex: Option<f32>,
    pub gap: Option<f32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Display {
    Flex,
    None,
    Block,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Stretch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn background_color(mut self, color: egui::Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn border(mut self, color: egui::Color32, width: f32) -> Self {
        self.border_color = Some(color);
        self.border_width = Some(width);
        self
    }

    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(radius);
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(egui::Vec2::new(padding, padding));
        self
    }

    pub fn padding_xy(mut self, x: f32, y: f32) -> Self {
        self.padding = Some(egui::Vec2::new(x, y));
        self
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = Some(egui::Vec2::new(margin, margin));
        self
    }

    pub fn margin_xy(mut self, x: f32, y: f32) -> Self {
        self.margin = Some(egui::Vec2::new(x, y));
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn display(mut self, display: Display) -> Self {
        self.display = Some(display);
        self
    }

    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.flex_direction = Some(direction);
        self
    }

    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.align_items = Some(align);
        self
    }

    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.justify_content = Some(justify);
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap);
        self
    }
}

pub struct StyleSheet {
    pub container: Rc<Style>,
    pub button_container: Rc<Style>,
}

impl StyleSheet {
    pub fn new() -> Self {
        Self {
            container: Rc::new(Style::new()),
            button_container: Rc::new(
                Style::new()
                    .flex_direction(FlexDirection::Row)
                    .gap(8.0)
                    .padding(8.0),
            ),
        }
    }

    pub fn custom<F>(f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        f()
    }
}

impl Default for StyleSheet {
    fn default() -> Self {
        Self::new()
    }
}
