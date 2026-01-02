use eframe::egui;

use crate::store;

store! {
  pub struct GlobalStore {
    is_open_settings: bool = false
  }
}

impl GlobalStore {
    pub fn toggle_is_open_settings(&self, _ctx: &egui::Context) {
        let mut reactive = self.reactive(_ctx);
        let current = *reactive.is_open_settings();
        *reactive.is_open_settings() = !current;
    }
}

pub fn global_store() -> std::cell::Ref<'static, GlobalStore> {
    GlobalStore::instance()
}
