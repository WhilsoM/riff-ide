use eframe::egui;

use crate::{core::models::Tab, store};

store! {
    #[derive(Debug, Clone)]
    pub struct TabsStore {
        tabs: Vec<Tab> = Vec::new(),
    }

    push_new_tab(&mut self, ctx: &egui::Context, tab: Tab) {
      let mut reactive = self.reactive(ctx);
      reactive.tabs().push(tab);
    }
}
