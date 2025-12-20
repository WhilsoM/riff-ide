use std::collections::HashMap;
use std::rc::Rc;

use eframe::egui;

use crate::core::enums::enums::Icon;
use crate::core::stores::icons::icons_services::IconsServicesStore;

pub struct IconsInteractionsStore {
    icons: HashMap<Icon, egui::TextureHandle>,
    services: Rc<IconsServicesStore>,
}

impl IconsInteractionsStore {
    pub fn new(ctx: &egui::Context) -> Self {
        let services = Rc::new(IconsServicesStore::new());
        let mut icons = HashMap::new();

        icons.insert(Icon::Folder, services.load_folder_icon(ctx));

        icons.insert(Icon::OpenFolder, services.load_open_folder_icon(ctx));

        icons.insert(Icon::File, services.load_file_icon(ctx));

        icons.insert(Icon::Rust, services.load_rust_icon(ctx));

        Self { icons, services }
    }

    pub fn get(&self, icon: &Icon) -> &egui::TextureHandle {
        &self.icons[icon]
    }

    pub fn has(&self, icon: &Icon) -> bool {
        self.icons.contains_key(icon)
    }

    pub fn get_all(&self) -> &HashMap<Icon, egui::TextureHandle> {
        &self.icons
    }
}
