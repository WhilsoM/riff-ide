#[macro_export]
macro_rules! on_click {
    ($func:path $(, $args:expr)*) => {{
        let f = move || $func($($args.clone()),*);
        Rc::new(f) as $crate::core::types::types::Handler
    }};
}
