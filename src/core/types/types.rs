use std::{cell::RefCell, rc::Rc};

use crate::core::{lib::rsx::Component, models::Entry};

pub type Handler = Rc<dyn Fn()>;

pub type Element = Rc<dyn Component>;

pub type EntryRc = Rc<RefCell<Entry>>;
