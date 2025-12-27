use eframe::egui;

use crate::store;

store! {
    pub struct HotkeysInteractionsStore {
      is_open_explorer: bool = true,
    }

    toggle_explorer(&mut self, ctx: &egui::Context) {
      let mut reactive = self.reactive(ctx);
      *reactive.is_open_explorer() = !*reactive.is_open_explorer();
      println!("IS OPEN EXPLORER: {:?}", self.is_open_explorer)
    }
}
