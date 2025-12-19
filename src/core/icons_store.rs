use std::collections::HashMap;

use eframe::egui;

use crate::core::{enums::enums::Icon, utils::utils::load_icon};

pub struct IconStore {
    icons: HashMap<Icon, egui::TextureHandle>,
}

impl IconStore {
    pub fn new(ctx: &egui::Context) -> Self {
        let mut icons = HashMap::new();

        icons.insert(
            Icon::Folder,
            load_icon(
                ctx,
                "folder",
                include_bytes!("../../assets/icons/folder.png"),
            ),
        );

        icons.insert(
            Icon::OpenFolder,
            load_icon(
                ctx,
                "open_folder",
                include_bytes!("../../assets/icons/folder-open.png"),
            ),
        );

        icons.insert(
            Icon::File,
            load_icon(
                ctx,
                "file",
                include_bytes!("../../assets/icons/file-text.png"),
            ),
        );

        icons.insert(
            Icon::Rust,
            load_icon(
                ctx,
                "rust",
                include_bytes!("../../assets/icons/rust-icon.png"),
            ),
        );

        Self { icons }
    }

    pub fn get(&self, icon: &Icon) -> &egui::TextureHandle {
        &self.icons[icon]
    }
}
