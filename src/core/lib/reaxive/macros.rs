#[macro_export]
macro_rules! store {
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $($fname:ident : $ftype:ty = $default:expr),* $(,)?
        }
        $(
            $method_name:ident ( $($params:tt)* ) $(-> $ret:ty)? $body:block
        )*
    ) => {
        $(#[$struct_meta])*
        $vis struct $name {
            $(pub $fname: crate::core::lib::reaxive::reactive::ReField<$ftype>),*
        }

        pub struct ReactiveAccess<'a> {
            store: &'a $name,
            ctx: &'a eframe::egui::Context,
            _changed: std::cell::Cell<bool>,
        }

        thread_local! {
            static INSTANCE: std::cell::RefCell<$name> =
                std::cell::RefCell::new($name {
                    $($fname: crate::core::lib::reaxive::reactive::ReField::new($default)),*
                });
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $($fname: crate::core::lib::reaxive::reactive::ReField::new($default)),*
                }
            }

            pub fn reactive<'a>(
                &'a self,
                ctx: &'a eframe::egui::Context
            ) -> ReactiveAccess<'a> {
                ReactiveAccess {
                    store: self,
                    ctx,
                    _changed: std::cell::Cell::new(false),
                }
            }

            pub fn instance() -> std::cell::Ref<'static, $name> {
                INSTANCE.with(|s| unsafe {
                    std::mem::transmute::<
                        std::cell::Ref<'_, $name>,
                        std::cell::Ref<'static, $name>
                    >(s.borrow())
                })
            }

            pub fn instance_mut() -> std::cell::RefMut<'static, $name> {
                INSTANCE.with(|s| unsafe {
                    std::mem::transmute::<
                        std::cell::RefMut<'_, $name>,
                        std::cell::RefMut<'static, $name>
                    >(s.borrow_mut())
                })
            }

            $(
                pub fn $method_name ( $($params)* ) $(-> $ret)? $body
            )*
        }

        impl<'a> ReactiveAccess<'a> {
            $(
                pub fn $fname(&mut self) -> std::cell::RefMut<'a, $ftype> {
                    let ctx = self.ctx.clone();
                    let cb = std::rc::Rc::new(move || ctx.request_repaint());
                    self.store.$fname.observer.subscribe(cb);
                    self._changed.set(true);
                    self.store.$fname.borrow_mut()
                }
            )*
        }

        impl<'a> Drop for ReactiveAccess<'a> {
            fn drop(&mut self) {
                if self._changed.get() {
                    $(
                        self.store.$fname.observer.notify();
                    )*
                    self.ctx.request_repaint();
                }
            }
        }
    };
}
