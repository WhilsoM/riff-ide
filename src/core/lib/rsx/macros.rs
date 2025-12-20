#[macro_export]
macro_rules! rsx {
    ($text:literal) => {
        {
            use std::rc::Rc;
            Rc::new($crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                ui.label($text);
            })) as Rc<dyn $crate::core::lib::rsx::component::Component>
        }
    };

    ($component:ident ( $($args:expr),* $(,)? )) => {
        {
            $component($($args),*)
        }
    };

    ($component:ident {}) => {
        {
            use std::rc::Rc;
            let component = $component::new();
            Rc::new($crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                use $crate::core::lib::rsx::component::Component;
                component.render(ui);
            })) as Rc<dyn $crate::core::lib::rsx::component::Component>
        }
    };

    (<> $($children:tt)* </>) => {
        {
            use std::rc::Rc;
            let children_vec = $crate::rsx_parse_children!($($children)*);
            Rc::new($crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                use $crate::core::lib::rsx::component::Component;
                for child in &children_vec {
                    child.render(ui);
                }
            })) as Rc<dyn $crate::core::lib::rsx::component::Component>
        }
    };

    (if ($condition:expr) { $($content:tt)* }) => {
        {
            use std::rc::Rc;
            if $condition {
                $crate::rsx!($($content)*)
            } else {
                Rc::new($crate::core::lib::rsx::component::ComponentWrapper::new(|_ui: &mut eframe::egui::Ui| {})) as Rc<dyn $crate::core::lib::rsx::component::Component>
            }
        }
    };

    (if ($condition:expr) { $($content_if:tt)* } else { $($content_else:tt)* }) => {
        {
            use std::rc::Rc;
            if $condition {
                $crate::rsx!($($content_if)*)
            } else {
                $crate::rsx!($($content_else)*)
            }
        }
    };

    (for $item:ident in ($array:expr) { $($content:tt)* }) => {
        {
            use std::rc::Rc;
            let mut children = vec![];
            for $item in $array {
                children.push($crate::rsx!($($content)*));
            }
            Rc::new($crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                use $crate::core::lib::rsx::component::Component;
                for child in &children {
                    child.render(ui);
                }
            })) as Rc<dyn $crate::core::lib::rsx::component::Component>
        }
    };

    (for ($idx:ident, $item:ident) in ($array:expr) { $($content:tt)* }) => {
        {
            use std::rc::Rc;
            let mut children = vec![];
            for ($idx, $item) in $array.iter().enumerate() {
                children.push($crate::rsx!($($content)*));
            }
            Rc::new($crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                use $crate::core::lib::rsx::component::Component;
                for child in &children {
                    child.render(ui);
                }
            })) as Rc<dyn $crate::core::lib::rsx::component::Component>
        }
    };

    (
        $component:ident {
            $($content:tt)*
        }
    ) => {
        {
            $crate::rsx_parse_component!($component, $($content)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! rsx_parse_component {
    (
        $component:ident,
        children: {
            $($child:tt)*
        }
    ) => {
        {
            use std::rc::Rc;
            let mut props = <$component as $crate::core::lib::rsx::component::ComponentWithProps>::Props::default();
            let children_vec = $crate::rsx_parse_children!($($child)*);
            props.children = $crate::core::lib::rsx::component::Children::Multiple(children_vec);
            let component = $component::new_with_props(props);
            Rc::new($crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                use $crate::core::lib::rsx::component::Component;
                component.render(ui);
            })) as Rc<dyn $crate::core::lib::rsx::component::Component>
        }
    };

    (
        $component:ident,
        $($rest:tt)*
    ) => {
        {
            use std::rc::Rc;
            let mut props = <$component as $crate::core::lib::rsx::component::ComponentWithProps>::Props::default();
            $crate::rsx_parse_props!(props, $($rest)*);
            let component = $component::new_with_props(props);
            Rc::new($crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                use $crate::core::lib::rsx::component::Component;
                component.render(ui);
            })) as Rc<dyn $crate::core::lib::rsx::component::Component>
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! rsx_parse_props {
    ($props:ident,) => {};
    ($props:ident) => {};

    (
        $props:ident,
        $key:ident : $value:expr,
        $($rest:tt)*
    ) => {
        $props.$key = $value;
        $crate::rsx_parse_props!($props, $($rest)*)
    };

    (
        $props:ident,
        children: {
            $($child:tt)*
        }
    ) => {
        {
            let children_vec = $crate::rsx_parse_children!($($child)*);
            $props.children = $crate::core::lib::rsx::component::Children::Multiple(children_vec);
        }
    };

    (
        $props:ident,
        children: {
            $($child:tt)*
        }
        $($rest:tt)*
    ) => {
        {
            let children_vec = $crate::rsx_parse_children!($($child)*);
            $props.children = $crate::core::lib::rsx::component::Children::Multiple(children_vec);
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! rsx_parse_children {
    () => {
        vec![]
    };

    // Function call with args and semicolon - must come before other rules
    (
        $component:ident ( $($args:expr),* $(,)? ) ; $($rest:tt)*
    ) => {
        {
            let mut vec = vec![$crate::rsx!($component ( $($args),* ))];
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    // Function call with args (final, no semicolon)
    (
        $component:ident ( $($args:expr),* $(,)? )
    ) => {
        {
            vec![$crate::rsx!($component ( $($args),* ))]
        }
    };

    (
        $component:ident {} ; $($rest:tt)*
    ) => {
        {
            let mut vec = vec![$crate::rsx!($component {})];
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        $component:ident {}
    ) => {
        {
            vec![$crate::rsx!($component {})]
        }
    };

    (
        $component:ident {} $($rest:tt)+
    ) => {
        {
            let mut vec = vec![$crate::rsx!($component {})];
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        $component:ident { $($content:tt)* } ; $($rest:tt)*
    ) => {
        {
            let mut vec = vec![$crate::rsx!($component { $($content)* })];
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        $component:ident { $($content:tt)* } $next:tt $($rest:tt)*
    ) => {
        {
            let mut vec = vec![$crate::rsx!($component { $($content)* })];
            vec.extend($crate::rsx_parse_children!($next $($rest)*));
            vec
        }
    };

    (
        $component:ident { $($content:tt)* }
    ) => {
        {
            vec![$crate::rsx!($component { $($content)* })]
        }
    };

    (
        $text:literal $(;)? $($rest:tt)*
    ) => {
        {
            let mut vec = vec![$crate::rsx!($text)];
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        $text:literal
    ) => {
        {
            vec![$crate::rsx!($text)]
        }
    };

    (
        if ($condition:expr) { $($content:tt)* } $(;)? $($rest:tt)*
    ) => {
        {
            let mut vec = vec![];
            let conditional = $crate::rsx!(if ($condition) { $($content)* });
            vec.push(conditional);
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        if ($condition:expr) { $($content_if:tt)* } else { $($content_else:tt)* } $(;)? $($rest:tt)*
    ) => {
        {
            let mut vec = vec![];
            let conditional = $crate::rsx!(if ($condition) { $($content_if)* } else { $($content_else)* });
            vec.push(conditional);
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        <> $($fragment_children:tt)* </> $(;)? $($rest:tt)*
    ) => {
        {
            let mut vec = vec![$crate::rsx!(<> $($fragment_children)* </>)];
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        for $item:ident in ($array:expr) { $($content:tt)* } $(;)? $($rest:tt)*
    ) => {
        {
            let mut vec = vec![];
            for $item in $array {
                vec.push($crate::rsx!($($content)*));
            }
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        for ($idx:ident, $item:ident) in ($array:expr) { $($content:tt)* } $(;)? $($rest:tt)*
    ) => {
        {
            let mut vec = vec![];
            for ($idx, $item) in $array.iter().enumerate() {
                vec.push($crate::rsx!($($content)*));
            }
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };
}
