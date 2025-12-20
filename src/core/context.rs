use std::cell::RefCell;
use std::rc::Rc;

use crate::core::stores::icons::IconsInteractionsStore;
use crate::modules::editor::stores::{
    FileActionsStore, FileInteractionsStore, ThemeInteractionsStore,
};

pub struct AppContext {
    pub icons: Rc<IconsInteractionsStore>,
    pub file_interactions: Rc<RefCell<FileInteractionsStore>>,
    pub file_actions: Rc<RefCell<FileActionsStore>>,
    pub theme: Rc<ThemeInteractionsStore>,
    pub egui_ctx: Rc<RefCell<Option<eframe::egui::Context>>>,
}

impl AppContext {
    pub fn new(
        icons: Rc<IconsInteractionsStore>,
        file_interactions: Rc<RefCell<FileInteractionsStore>>,
        file_actions: Rc<RefCell<FileActionsStore>>,
        theme: Rc<ThemeInteractionsStore>,
    ) -> Self {
        Self {
            icons,
            file_interactions,
            file_actions,
            theme,
            egui_ctx: Rc::new(RefCell::new(None)),
        }
    }

    pub fn request_repaint(&self) {
        if let Some(ctx) = self.egui_ctx.borrow().as_ref() {
            ctx.request_repaint();
        }
    }
}

thread_local! {
    static APP_CONTEXT: RefCell<Option<Rc<AppContext>>> = RefCell::new(None);
}

pub fn init_context(context: Rc<AppContext>) {
    APP_CONTEXT.with(|ctx| {
        *ctx.borrow_mut() = Some(context);
    });
}

pub fn get_context() -> Rc<AppContext> {
    APP_CONTEXT.with(|ctx| {
        ctx.borrow()
            .as_ref()
            .expect("AppContext не инициализирован. Вызовите init_context() перед использованием.")
            .clone()
    })
}

pub fn with_context<F, R>(f: F) -> R
where
    F: FnOnce(&AppContext) -> R,
{
    APP_CONTEXT.with(|ctx| {
        let context = ctx
            .borrow()
            .as_ref()
            .expect("AppContext не инициализирован. Вызовите init_context() перед использованием.")
            .clone();
        f(&context)
    })
}

pub fn try_get_context() -> Option<Rc<AppContext>> {
    APP_CONTEXT.with(|ctx| ctx.borrow().as_ref().map(|c| c.clone()))
}
