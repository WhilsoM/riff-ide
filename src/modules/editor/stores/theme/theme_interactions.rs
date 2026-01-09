use crate::core::ui::ui_kit::style::Style;
use eframe::egui;
use std::rc::Rc;

use crate::store;

store! {
    pub struct ThemeInteractionsStore {
        bg_main_100: egui::Color32 = egui::Color32::from_rgb(25, 25, 25),
        bg_main_200: egui::Color32 = egui::Color32::from_rgb(25, 25, 25),
        bg_main_300: egui::Color32 = egui::Color32::from_rgb(30, 30, 30),
        bg_hover: egui::Color32 = egui::Color32::from_rgb(45, 45, 45),
        bg_active: egui::Color32 = egui::Color32::from_rgb(55, 55, 55),
        bg_selected: egui::Color32 = egui::Color32::from_rgb(0, 122, 204),

        text_primary: egui::Color32 = egui::Color32::from_rgb(255, 255, 255),
        text_secondary: egui::Color32 = egui::Color32::from_rgb(170, 170, 170),
        text_tertiary: egui::Color32 = egui::Color32::from_rgb(130, 130, 130),
        text_disabled: egui::Color32 = egui::Color32::from_rgb(100, 100, 100),

        accent_primary: egui::Color32 = egui::Color32::from_rgb(0, 122, 204),
        accent_secondary: egui::Color32 = egui::Color32::from_rgb(0, 151, 251),
        accent_hover: egui::Color32 = egui::Color32::from_rgb(0, 100, 180),

        border_primary: egui::Color32 = egui::Color32::from_rgb(60, 60, 60),
        border_secondary: egui::Color32 = egui::Color32::from_rgb(50, 50, 50),

        error: egui::Color32 = egui::Color32::from_rgb(244, 67, 54),
        warning: egui::Color32 = egui::Color32::from_rgb(255, 152, 0),
        success: egui::Color32 = egui::Color32::from_rgb(76, 175, 80),
        info: egui::Color32 = egui::Color32::from_rgb(33, 150, 243),
    }
}

impl ThemeInteractionsStore {
    // BACKGROUND
    pub fn bg_main_100_style(&self, ctx: &egui::Context) -> Rc<Style> {
        Rc::new(Style::new().background_color(self.bg_main_100.get(ctx)))
    }

    pub fn bg_main_200_style(&self, ctx: &egui::Context) -> Rc<Style> {
        Rc::new(Style::new().background_color(self.bg_main_200.get(ctx)))
    }

    pub fn bg_main_300_style(&self, ctx: &egui::Context) -> Rc<Style> {
        Rc::new(Style::new().background_color(self.bg_main_300.get(ctx)))
    }

    pub fn bg_hover_style(&self, ctx: &egui::Context) -> Rc<Style> {
        Rc::new(Style::new().background_color(self.bg_hover.get(ctx)))
    }

    // COLOR
    pub fn text_primary_style(&self, ctx: &egui::Context) -> Rc<Style> {
        Rc::new(Style::new().color(self.bg_hover.get(ctx)))
    }
}

pub fn theme_store() -> std::cell::Ref<'static, ThemeInteractionsStore> {
    ThemeInteractionsStore::instance()
}
