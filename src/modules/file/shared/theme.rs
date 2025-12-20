use crate::core::ui::ui_kit::style::{Style, StyleSheet};
use eframe::egui;
use std::rc::Rc;

pub struct FileTheme {
    stylesheet: StyleSheet,
}

impl FileTheme {
    pub fn new() -> Self {
        let bg_primary = egui::Color32::from_rgb(30, 30, 30);
        let bg_secondary = egui::Color32::from_rgb(25, 25, 25);
        let _bg_tertiary = egui::Color32::from_rgb(20, 20, 20);
        let _border = egui::Color32::from_rgb(60, 60, 60);
        let _text_primary = egui::Color32::from_rgb(212, 212, 212);
        let _text_secondary = egui::Color32::from_rgb(170, 170, 170);
        let _accent = egui::Color32::from_rgb(0, 122, 204);
        let hover = egui::Color32::from_rgb(45, 45, 45);

        let mut stylesheet = StyleSheet::new();
        stylesheet.explorer_panel = Some(Rc::new(
            Style::new().background_color(bg_secondary).width(250.0),
        ));
        stylesheet.editor_panel = Some(Rc::new(Style::new().background_color(bg_primary)));
        stylesheet.file_item = Some(Rc::new(Style::new().padding(4.0)));
        stylesheet.file_item_hover =
            Some(Rc::new(Style::new().background_color(hover).padding(4.0)));
        stylesheet.text_primary = Some(Rc::new(Style::new()));
        stylesheet.text_secondary = Some(Rc::new(Style::new()));

        Self { stylesheet }
    }

    pub fn stylesheet(&self) -> &StyleSheet {
        &self.stylesheet
    }

    pub fn background_primary() -> egui::Color32 {
        egui::Color32::from_rgb(30, 30, 30)
    }

    pub fn background_secondary() -> egui::Color32 {
        egui::Color32::from_rgb(25, 25, 25)
    }

    pub fn text_primary() -> egui::Color32 {
        egui::Color32::from_rgb(212, 212, 212)
    }

    pub fn text_secondary() -> egui::Color32 {
        egui::Color32::from_rgb(170, 170, 170)
    }

    pub fn accent() -> egui::Color32 {
        egui::Color32::from_rgb(0, 122, 204)
    }

    pub fn border() -> egui::Color32 {
        egui::Color32::from_rgb(60, 60, 60)
    }
}
