use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use eframe::egui;

use crate::core::enums::enums::UiAction;
use crate::core::models::Entry;
use crate::core::stores::app_name_store::AppNameStore;
use crate::core::stores::icons::IconsInteractionsStore;
use crate::core::utils::utils::read_current_folder;
use crate::modules::file::stores::{
    FileActionsStore, FileInteractionsStore, ThemeInteractionsStore,
};
use crate::modules::file::{CodeEditor, FileExplorer};
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
    theme: Rc<ThemeInteractionsStore>,
    is_dirty: bool,
    // ra_process: Option<std::process::Child>,
    // lsp: Option<Arc<AsyncMutex<LspClient>>>,
    // diagnostics: Arc<Mutex<Vec<Diagnostic>>>,
    // rt: Runtime,
}

impl MyApp {
    pub fn new(
        icons: IconsInteractionsStore,
        opened_file: Option<PathBuf>,
        _opened_text: String,
        app_name_store: AppNameStore,
    ) -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let files = read_current_folder(&current_dir);
        let actions_store = Rc::new(RefCell::new(ActionsStore::new()));

        let file_actions = Rc::new(RefCell::new(FileActionsStore::new()));
        let file_interactions = Rc::new(RefCell::new(FileInteractionsStore::new()));
        let theme = Rc::new(ThemeInteractionsStore::new());

        if let Some(path) = opened_file.as_ref() {
            file_actions.borrow_mut().open_file(path);
        }

        Self {
            current_dir,
            files,
            icons: Rc::new(icons),
            actions_store,
            app_name_store,
            file_actions,
            file_interactions,
            theme,
            is_dirty: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.dark_mode = true;
        visuals.panel_fill = self.theme.bg_main_200;
        visuals.window_fill = self.theme.bg_main_100;
        visuals.faint_bg_color = self.theme.bg_main_300;
        visuals.extreme_bg_color = self.theme.bg_main_100;
        visuals.code_bg_color = self.theme.bg_main_100;
        visuals.window_stroke.color = self.theme.border_primary;
        visuals.window_stroke.width = 1.0;
        visuals.widgets.noninteractive.bg_fill = self.theme.bg_main_200;
        visuals.widgets.noninteractive.weak_bg_fill = self.theme.bg_main_300;
        visuals.widgets.inactive.bg_fill = self.theme.bg_main_200;
        visuals.widgets.hovered.bg_fill = self.theme.bg_hover;
        visuals.widgets.active.bg_fill = self.theme.bg_active;
        visuals.widgets.open.bg_fill = self.theme.bg_selected;
        visuals.override_text_color = Some(self.theme.text_primary);
        visuals.selection.bg_fill = self.theme.bg_selected;
        visuals.selection.stroke.color = self.theme.accent_primary;
        visuals.hyperlink_color = self.theme.accent_primary;
        ctx.set_visuals(visuals);

        if let Some(UiAction::OpenFile(path)) = self.file_interactions.borrow_mut().take_action() {
            self.file_actions.borrow_mut().open_file(&path);
        }

        let files_rc = Rc::new(RefCell::new(self.files.clone()));

        let explorer = FileExplorer::new(
            files_rc.clone(),
            self.icons.clone(),
            self.file_interactions.clone(),
            self.file_actions.clone(),
            self.theme.clone(),
        );
        explorer.render(ctx);

        self.files = files_rc.borrow().clone();

        let opened_file = self.file_actions.borrow().opened_file.borrow().clone();
        let opened_text = self.file_actions.borrow().opened_text.clone();
        let editor = CodeEditor::new(
            opened_file,
            opened_text,
            self.actions_store.clone(),
            self.theme.clone(),
        );
        editor.render(ctx);
    }
}
