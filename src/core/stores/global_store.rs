use crate::{core::lib::reaxive::reactive::ReField, store};
use eframe::egui;

store! {
  pub struct GlobalStore {
    is_open_settings: bool = false,
    font_size: f32 = 17.5
  }
}

impl GlobalStore {
    pub fn toggle_is_open_settings(&self, _ctx: &egui::Context) {
        let mut reactive = self.reactive(_ctx);
        let current = *reactive.is_open_settings();
        *reactive.is_open_settings() = !current;
    }

    pub fn get_font_size(&self) -> ReField<f32> {
        self.font_size.clone()
    }

    pub fn change_font_size(&self, _ctx: &egui::Context, size: f32) {
        let mut reactive = self.reactive(_ctx);
        println!("CURRENT FONT SIZE: {:?}", self.font_size);
        *reactive.font_size() = size;
        println!("FONT SIZE AFTER CHANGE: {:?}", self.font_size)
    }
}

pub fn global_store() -> std::cell::Ref<'static, GlobalStore> {
    GlobalStore::instance()
}
