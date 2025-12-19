#[macro_export]
macro_rules! store {
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $($fname:ident : $ftype:ty = $default:expr),* $(,)?
        }
        $($method_name:ident (&self $(, $arg:ident : $arg_type:ty)*) $body:block)*
    ) => {
        $(#[$struct_meta])*
        $vis struct $name {
            $(pub $fname: crate::core::lib::reaxive::reactive::ReField<$ftype>),*
        }

        impl $name {
            /// Конструктор: автоматически делает все поля реактивными
            pub fn new() -> Self {
                Self {
                    $($fname: crate::core::lib::reaxive::reactive::ReField::new($default)),*
                }
            }

            $(
                pub fn $method_name(&self $(, $arg : $arg_type)*) $body
            )*
        }
    };
}
