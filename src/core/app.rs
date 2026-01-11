use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use eframe::egui;

use crate::core::enums::enums::{Hotkeys, UiAction};
use crate::core::stores::app_name_store::AppNameStore;
use crate::core::stores::global_store::{global_store, GlobalStore};
use crate::core::stores::icons::IconsInteractionsStore;
use crate::core::types::types::EntryRc;
use crate::core::ui::ui_kit::render_app;
use crate::core::utils::utils::read_current_folder;
use crate::modules::editor::components::App;
use crate::modules::editor::stores::hotkeys::HotkeysInteractionsStore;
use crate::modules::editor::stores::{
    EditorInteractionsStore, FileActionsStore, FileInteractionsStore, ThemeInteractionsStore,
};

pub struct MyApp {
    current_dir: PathBuf,
    files: Vec<EntryRc>,
    icons: Rc<IconsInteractionsStore>,
    app_name_store: AppNameStore,
    file_actions: Rc<RefCell<FileActionsStore>>,
    file_interactions: Rc<RefCell<FileInteractionsStore>>,
    editor_interactions: Rc<RefCell<EditorInteractionsStore>>,
    hotkeys_interactions: Rc<RefCell<HotkeysInteractionsStore>>,
    theme: Rc<ThemeInteractionsStore>,
    pending_actions: Vec<Hotkeys>,
    global_store: Rc<RefCell<GlobalStore>>,
}

impl MyApp {
    pub fn new(
        icons: IconsInteractionsStore,
        _opened_file: Option<PathBuf>,
        _opened_text: String,
        app_name_store: AppNameStore,
    ) -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let files = read_current_folder(&current_dir);

        let file_actions = Rc::new(RefCell::new(FileActionsStore::new()));
        let file_interactions = Rc::new(RefCell::new(FileInteractionsStore::new()));
        let editor_interactions = Rc::new(RefCell::new(EditorInteractionsStore::new()));
        let theme = Rc::new(ThemeInteractionsStore::new());
        let hotkeys_interactions = Rc::new(RefCell::new(HotkeysInteractionsStore::new()));
        let global_store = Rc::new(RefCell::new(GlobalStore::new()));

        Self {
            current_dir,
            files,
            icons: Rc::new(icons),
            app_name_store,
            file_actions,
            file_interactions,
            editor_interactions,
            theme,
            hotkeys_interactions,
            pending_actions: Vec::new(),
            global_store,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let delta = ctx.input(|i| i.raw_scroll_delta.y);

        // change font size
        if ctx.input(|i| i.modifiers.command) && delta != 0.0 {
            let font_size_field = global_store().get_font_size();

            let current_size = font_size_field.get(ctx);

            let step = 0.5;
            let mut new_size = if delta > 0.0 {
                current_size + step
            } else {
                current_size - step
            };

            new_size = new_size.clamp(8.0, 72.0);
            font_size_field.set(new_size);

            global_store().change_font_size(ctx, new_size);
        }

        use crate::modules::editor::stores::theme_store;
        let theme = theme_store();
        let mut visuals = egui::Visuals::dark();
        visuals.dark_mode = true;
        visuals.panel_fill = theme.bg_main_200.get(ctx);
        visuals.window_fill = theme.bg_main_100.get(ctx);
        visuals.faint_bg_color = theme.bg_main_300.get(ctx);
        visuals.extreme_bg_color = theme.bg_main_100.get(ctx);
        visuals.code_bg_color = theme.bg_main_100.get(ctx);
        visuals.window_stroke.color = theme.border_primary.get(ctx);
        visuals.window_stroke.width = 1.0;
        visuals.widgets.noninteractive.bg_fill = theme.bg_main_200.get(ctx);
        visuals.widgets.noninteractive.weak_bg_fill = theme.bg_main_300.get(ctx);
        visuals.widgets.inactive.bg_fill = theme.bg_main_200.get(ctx);
        visuals.widgets.hovered.bg_fill = theme.bg_hover.get(ctx);
        visuals.widgets.active.bg_fill = theme.bg_active.get(ctx);
        visuals.widgets.open.bg_fill = theme.bg_selected.get(ctx);
        visuals.override_text_color = Some(theme.text_primary.get(ctx));
        visuals.selection.bg_fill = theme.bg_selected.get(ctx);
        visuals.selection.stroke.color = theme.accent_primary.get(ctx);
        visuals.hyperlink_color = theme.accent_primary.get(ctx);
        ctx.set_visuals(visuals);

        use crate::modules::editor::stores::editor_interactions_store;
        if let Some(UiAction::OpenFile(path)) = self.file_interactions.borrow_mut().take_action() {
            editor_interactions_store().open_tab(ctx, path);
        }

        use crate::modules::editor::stores::context::{set_all_stores, AppStores};

        let files_rc = Rc::new(RefCell::new(self.files.clone()));
        set_all_stores(AppStores {
            editor_interactions: self.editor_interactions.clone(),
            theme: self.theme.clone(),
            file_interactions: self.file_interactions.clone(),
            file_actions: self.file_actions.clone(),
            icons: self.icons.clone(),
            files: files_rc.clone(),
            hotkeys_interactions: self.hotkeys_interactions.clone(),
            global_store: self.global_store.clone(),
        });

        let app = App(ctx.clone());

        render_app(ctx, app);
    }
}
