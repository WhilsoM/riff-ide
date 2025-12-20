use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use eframe::egui;

use crate::core::enums::enums::UiAction;
use crate::core::models::Entry;
use crate::core::stores::app_name_store::AppNameStore;
use crate::core::stores::icons::IconsInteractionsStore;
use crate::core::ui::ui_kit::render_app;
use crate::core::utils::utils::read_current_folder;
use crate::modules::editor::components::App;
use crate::modules::editor::stores::{
    EditorInteractionsStore, FileActionsStore, FileInteractionsStore, ThemeInteractionsStore,
};
use crate::store;

#[derive(Debug, Clone)]
pub struct Counter {
    pub counter: usize,
}

store! {
    #[derive(Debug)]
    pub struct ActionsStore {
        items: Vec<Counter> = vec![Counter{counter:1}, Counter{counter:2}, Counter{counter:3}],
        counter: u32 = 0,
    }

    increment(&self, ctx: &egui::Context) {
        let mut reactive = self.reactive(ctx);
        *reactive.counter() += 1;
    }

    update_item(&self, ctx: &egui::Context, i: usize) {
        let mut reactive = self.reactive(ctx);
        if let Some(elem) = reactive.items().get_mut(i) {
            elem.counter += 1;
        }
    }
}

pub struct MyApp {
    current_dir: PathBuf,
    files: Vec<Entry>,
    icons: Rc<IconsInteractionsStore>,
    actions_store: Rc<RefCell<ActionsStore>>,
    app_name_store: AppNameStore,
    file_actions: Rc<RefCell<FileActionsStore>>,
    file_interactions: Rc<RefCell<FileInteractionsStore>>,
    editor_interactions: Rc<RefCell<EditorInteractionsStore>>,
    theme: Rc<ThemeInteractionsStore>,
    is_dirty: bool,
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
        let actions_store = Rc::new(RefCell::new(ActionsStore::new()));

        let file_actions = Rc::new(RefCell::new(FileActionsStore::new()));
        let file_interactions = Rc::new(RefCell::new(FileInteractionsStore::new()));
        let editor_interactions = Rc::new(RefCell::new(EditorInteractionsStore::new()));
        let theme = Rc::new(ThemeInteractionsStore::new());

        Self {
            current_dir,
            files,
            icons: Rc::new(icons),
            actions_store,
            app_name_store,
            file_actions,
            file_interactions,
            editor_interactions,
            theme,
            is_dirty: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
        use crate::modules::editor::stores::file::file_actions::file_actions_store;
        if let Some(UiAction::OpenFile(path)) = self.file_interactions.borrow_mut().take_action() {
            file_actions_store().open_file(ctx, path.clone());
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
        });

        let app = App(ctx.clone());

        render_app(ctx, app);

        self.files = files_rc.borrow().clone();
    }
}
