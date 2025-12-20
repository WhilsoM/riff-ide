use eframe::egui;

pub trait ComponentLifecycle {
    fn on_mount(&mut self, _ctx: &egui::Context) {}

    fn on_update(&mut self, _ctx: &egui::Context) {}

    fn on_unmount(&mut self, _ctx: &egui::Context) {}

    fn on_error(&mut self, _error: &dyn std::error::Error, _ctx: &egui::Context) {}
}

pub struct LifecycleWrapper<C> {
    component: C,
    is_mounted: bool,
}

impl<C> LifecycleWrapper<C> {
    pub fn new(component: C) -> Self {
        Self {
            component,
            is_mounted: false,
        }
    }
}

impl<C> LifecycleWrapper<C>
where
    C: crate::core::lib::rsx::component::Component + ComponentLifecycle,
{
    pub fn render_with_lifecycle(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if !self.is_mounted {
            self.component.on_mount(ctx);
            self.is_mounted = true;
        }

        self.component.on_update(ctx);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.component.render(ui);
        }));

        if let Err(err) = result {
            let error_msg = format!("Component render error: {:?}", err);
            self.component.on_error(
                &std::io::Error::new(std::io::ErrorKind::Other, error_msg),
                ctx,
            );
        }
    }
}
