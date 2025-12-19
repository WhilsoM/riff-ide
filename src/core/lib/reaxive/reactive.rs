use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use eframe::egui;

#[derive(Default)]
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

pub struct ReField<T> {
    value: RefCell<T>,
    observer: Observer,
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
}
