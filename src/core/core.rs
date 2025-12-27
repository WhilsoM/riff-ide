use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use eframe::egui;

use crate::core::enums::enums::{Hotkeys, UiAction};
use crate::core::models::EntryRc;
use crate::core::stores::app_name_store::AppNameStore;
use crate::core::stores::icons::IconsInteractionsStore;
use crate::core::ui::ui_kit::render_app;
use crate::core::utils::utils::read_current_folder;
use crate::modules::editor::components::App;
use crate::modules::editor::stores::hotkeys::{HotkeysInteractionsStore, hotkeys_interactions};
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
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let toggle_explorer = ctx.input(|i| i.modifiers.mac_cmd && i.key_pressed(egui::Key::B));
        println!("Toggle explorer hotkey pressed: {}", toggle_explorer);

        if toggle_explorer {
            self.pending_actions.push(Hotkeys::ToggleExplorer);
        }

        // CLEAR PENDING ACTIONS
        for action in self.pending_actions.drain(..) {
            match action {
                Hotkeys::ToggleExplorer => {
                    self.hotkeys_interactions.borrow_mut().toggle_explorer(ctx);
                }
                _ => {}
            }
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

        use crate::modules::editor::stores::context::{AppStores, set_all_stores};

        let files_rc = Rc::new(RefCell::new(self.files.clone()));
        set_all_stores(AppStores {
            editor_interactions: self.editor_interactions.clone(),
            theme: self.theme.clone(),
            file_interactions: self.file_interactions.clone(),
            file_actions: self.file_actions.clone(),
            icons: self.icons.clone(),
            files: files_rc.clone(),
            hotkeys_interactions: self.hotkeys_interactions.clone(),
        });

        let app = App(ctx.clone());

        render_app(ctx, app);
    }
}
