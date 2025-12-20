use eframe::egui;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::core::lib::rsx::component::Component;

pub struct ComponentCache {
    cache: RefCell<HashMap<u64, Rc<dyn Component>>>,
}

impl ComponentCache {
    pub fn new() -> Self {
        Self {
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_or_insert<F>(&self, key: u64, factory: F) -> Rc<dyn Component>
    where
        F: FnOnce() -> Rc<dyn Component>,
    {
        let mut cache = self.cache.borrow_mut();
        cache.entry(key).or_insert_with(|| factory()).clone()
    }

    pub fn clear(&self) {
        self.cache.borrow_mut().clear();
    }

    pub fn remove(&self, key: u64) {
        self.cache.borrow_mut().remove(&key);
    }
}

impl Default for ComponentCache {
    fn default() -> Self {
        Self::new()
    }
}

pub fn compute_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub struct MemoizedComponent {
    component: Rc<dyn Component>,
    cache_key: u64,
}

impl MemoizedComponent {
    pub fn new(component: Rc<dyn Component>, cache_key: u64) -> Self {
        Self {
            component,
            cache_key,
        }
    }
}

impl Component for MemoizedComponent {
    fn render(&self, ui: &mut egui::Ui) {
        self.component.render(ui);
    }
}

#[macro_export]
macro_rules! memo {
    ($key:expr, $component:expr) => {{
        use std::cell::RefCell;
        use std::rc::Rc;
        use $crate::core::lib::rsx::memo::{compute_hash, ComponentCache, MemoizedComponent};
        thread_local! {
            static CACHE: RefCell<ComponentCache> = RefCell::new(ComponentCache::new());
        }

        let key = compute_hash(&$key);
        CACHE.with(|cache| {
            let cache = cache.borrow();
            let component = cache.get_or_insert(key, || $component);
            Rc::new(MemoizedComponent::new(component, key))
                as Rc<dyn $crate::core::lib::rsx::component::Component>
        })
    }};
}
