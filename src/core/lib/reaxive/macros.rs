#[macro_export]
macro_rules! store {
    (
        $vis:vis struct $name:ident {
            $($fname:ident : $ftype:ty = $default:expr),* $(,)?
        }
        $($method_name:ident ( $($arg:tt)* ) $body:block)*
    ) => {
        $vis struct $name {
            $(pub $fname: crate::core::lib::reaxive::reactive::ReField<$ftype>),*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $($fname: crate::core::lib::reaxive::reactive::ReField::new($default)),*
                }
            }

            $(
                pub fn $method_name($($arg)*) $body
            )*
        }
    };
}
