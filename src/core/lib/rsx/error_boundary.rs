use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::lib::rsx::component::Component;

pub struct ErrorBoundary {
    children: Rc<dyn Component>,
    fallback: Option<Rc<dyn Component>>,
    has_error: Rc<RefCell<bool>>,
    error_message: Rc<RefCell<Option<String>>>,
}

impl ErrorBoundary {
    pub fn new(children: Rc<dyn Component>) -> Self {
        Self {
            children,
            fallback: None,
            has_error: Rc::new(RefCell::new(false)),
            error_message: Rc::new(RefCell::new(None)),
        }
    }

    pub fn with_fallback(mut self, fallback: Rc<dyn Component>) -> Self {
        self.fallback = Some(fallback);
        self
    }
}

impl Component for ErrorBoundary {
    fn render(&self, ui: &mut egui::Ui) {
        if *self.has_error.borrow() {
            if let Some(fallback) = &self.fallback {
                fallback.render(ui);
            } else {
                ui.vertical(|ui| {
                    ui.heading("Something went wrong");
                    if let Some(ref error_msg) = *self.error_message.borrow() {
                        ui.label(error_msg);
                    }
                    if ui.button("Retry").clicked() {
                        *self.has_error.borrow_mut() = false;
                        *self.error_message.borrow_mut() = None;
                    }
                });
            }
            return;
        }

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.children.render(ui);
        }));

        if let Err(_err) = result {
            *self.has_error.borrow_mut() = true;
            *self.error_message.borrow_mut() = Some("Component render failed".to_string());
            if let Some(fallback) = &self.fallback {
                fallback.render(ui);
            } else {
                ui.label("Error: Component render failed");
            }
        }
    }
}

#[macro_export]
macro_rules! error_boundary {
    ($children:expr) => {{
        use std::rc::Rc;
        use $crate::core::lib::rsx::error_boundary::ErrorBoundary;
        Rc::new(ErrorBoundary::new($children))
            as Rc<dyn $crate::core::lib::rsx::component::Component>
    }};

    ($children:expr, fallback: $fallback:expr) => {{
        use std::rc::Rc;
        use $crate::core::lib::rsx::error_boundary::ErrorBoundary;
        Rc::new(ErrorBoundary::new($children).with_fallback($fallback))
            as Rc<dyn $crate::core::lib::rsx::component::Component>
    }};
}
