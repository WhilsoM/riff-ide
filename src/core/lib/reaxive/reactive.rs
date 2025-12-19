use eframe::egui;
use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

#[derive(Default, Debug, Clone)]
pub struct Observer {
    subscribers: RefCell<Vec<Weak<dyn Fn()>>>,
}

impl Observer {
    pub fn new() -> Self {
        Self {
            subscribers: RefCell::new(Vec::new()),
        }
    }

    pub fn notify(&self) {
        let subs = self.subscribers.borrow();
        for sub in subs.iter() {
            if let Some(cb) = sub.upgrade() {
                cb();
            }
        }
    }

    pub fn subscribe(&self, cb: Rc<dyn Fn()>) {
        self.subscribers.borrow_mut().push(Rc::downgrade(&cb));
    }
}
#[derive(Debug, Clone)]
pub struct ReField<T> {
    value: RefCell<T>,
    pub(crate) observer: Observer,
}

impl<T> ReField<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: RefCell::new(value),
            observer: Observer::new(),
        }
    }

    pub fn get(&self, ctx: &egui::Context) -> T
    where
        T: Clone,
    {
        let ctx = ctx.clone();
        let cb = Rc::new(move || ctx.request_repaint());
        self.observer.subscribe(cb);
        self.value.borrow().clone()
    }

    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        self.observer.notify();
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, T> {
        self.value.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, T> {
        self.value.borrow_mut()
    }
}

impl<T> ReField<Vec<T>> {
    pub fn update<F>(&self, ctx: &egui::Context, f: F)
    where
        F: FnOnce(&mut Vec<T>),
    {
        f(&mut *self.value.borrow_mut());
        self.observer.notify();
        ctx.request_repaint();
    }
}

pub struct ReactiveAccess<'a, T> {
    field: &'a ReField<T>,
    ctx: &'a egui::Context,
}

impl<'a, T> ReactiveAccess<'a, T> {
    fn new(field: &'a ReField<T>, ctx: &'a egui::Context) -> Self {
        let ctx_clone = ctx.clone();
        let cb = Rc::new(move || ctx_clone.request_repaint());
        field.observer.subscribe(cb);
        Self { field, ctx }
    }
}

impl<'a, T> Deref for ReactiveAccess<'a, T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.field.value
    }
}

impl<'a, T> Drop for ReactiveAccess<'a, T> {
    fn drop(&mut self) {
        self.field.observer.notify();
        self.ctx.request_repaint();
    }
}

impl<T> ReField<T> {
    pub fn with_ctx<'a>(&'a self, ctx: &'a egui::Context) -> ReactiveAccess<'a, T> {
        ReactiveAccess::new(self, ctx)
    }
}
