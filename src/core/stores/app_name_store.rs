use eframe::egui;

use crate::store;

store! {
    #[derive(Debug, Clone)]
    pub struct AppNameStore {
        app_name: Option<String> = Some(String::new()),
    }

    update_name(&mut self, ctx: &egui::Context, name: String) {
      let mut reactive = self.reactive(ctx);
      *reactive.app_name() = Some(name);
    }
}
