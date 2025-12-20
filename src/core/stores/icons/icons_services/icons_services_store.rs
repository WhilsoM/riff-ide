use eframe::egui;
use crate::core::utils::utils::load_icon;

pub struct IconsServicesStore;

impl IconsServicesStore {
    pub fn new() -> Self {
        Self
    }

    pub fn load_icon_from_bytes(
        &self,
        ctx: &egui::Context,
        name: &str,
        bytes: &[u8],
    ) -> egui::TextureHandle {
        load_icon(ctx, name, bytes)
    }

    pub fn load_folder_icon(&self, ctx: &egui::Context) -> egui::TextureHandle {
        self.load_icon_from_bytes(
            ctx,
            "folder",
            include_bytes!("../../../../../assets/icons/folder.png"),
        )
    }

    pub fn load_open_folder_icon(&self, ctx: &egui::Context) -> egui::TextureHandle {
        self.load_icon_from_bytes(
            ctx,
            "open_folder",
            include_bytes!("../../../../../assets/icons/folder-open.png"),
        )
    }

    pub fn load_file_icon(&self, ctx: &egui::Context) -> egui::TextureHandle {
        self.load_icon_from_bytes(
            ctx,
            "file",
            include_bytes!("../../../../../assets/icons/file-text.png"),
        )
    }

    pub fn load_rust_icon(&self, ctx: &egui::Context) -> egui::TextureHandle {
        self.load_icon_from_bytes(
            ctx,
            "rust",
            include_bytes!("../../../../../assets/icons/rust-icon.png"),
        )
    }
}

impl Default for IconsServicesStore {
    fn default() -> Self {
        Self::new()
    }
}

