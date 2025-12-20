#[macro_export]
macro_rules! rsx {
    ($text:literal) => {
        $crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
            ui.label($text);
        })
    };

    ($component:ident ( $($args:expr),* $(,)? )) => {
        {
            let component = $component($($args),*);
            $crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                component.render(ui);
            })
        }
    };

    ($component:ident {}) => {
        {
            let component = $component::new();
            $crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                component.render(ui);
            })
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
            let mut props = <$component as $crate::core::lib::rsx::component::ComponentWithProps>::Props::default();
            let children_vec = $crate::rsx_parse_children!($($child)*);
            props.children = $crate::core::lib::rsx::component::Children::Multiple(children_vec);
            let component = $component::new_with_props(props);
            $crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                component.render(ui);
            })
        }
    };

    (
        $component:ident,
        $($rest:tt)*
    ) => {
        {
            let mut props = <$component as $crate::core::lib::rsx::component::ComponentWithProps>::Props::default();
            $crate::rsx_parse_props!(props, $($rest)*);
            let component = $component::new_with_props(props);
            $crate::core::lib::rsx::component::ComponentWrapper::new(move |ui: &mut eframe::egui::Ui| {
                component.render(ui);
            })
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

    (
        $component:ident {} ; $($rest:tt)*
    ) => {
        {
            let mut vec = vec![Box::new($crate::rsx!($component {})) as Box<dyn $crate::core::lib::rsx::component::Component>];
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        $component:ident {}
    ) => {
        {
            vec![Box::new($crate::rsx!($component {})) as Box<dyn $crate::core::lib::rsx::component::Component>]
        }
    };

    (
        $component:ident { $($content:tt)* } ; $($rest:tt)*
    ) => {
        {
            let mut vec = vec![Box::new($crate::rsx!($component { $($content)* })) as Box<dyn $crate::core::lib::rsx::component::Component>];
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        $component:ident { $($content:tt)* }
    ) => {
        {
            vec![Box::new($crate::rsx!($component { $($content)* })) as Box<dyn $crate::core::lib::rsx::component::Component>]
        }
    };

    (
        $text:literal ; $($rest:tt)*
    ) => {
        {
            let mut vec = vec![Box::new($crate::rsx!($text)) as Box<dyn $crate::core::lib::rsx::component::Component>];
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        $text:literal
    ) => {
        {
            vec![Box::new($crate::rsx!($text)) as Box<dyn $crate::core::lib::rsx::component::Component>]
        }
    };

    (
        $component:ident ( $($args:expr),* $(,)? ) ; $($rest:tt)*
    ) => {
        {
            let mut vec = vec![Box::new($crate::rsx!($component ( $($args),* ))) as Box<dyn $crate::core::lib::rsx::component::Component>];
            vec.extend($crate::rsx_parse_children!($($rest)*));
            vec
        }
    };

    (
        $component:ident ( $($args:expr),* $(,)? )
    ) => {
        {
            vec![Box::new($crate::rsx!($component ( $($args),* ))) as Box<dyn $crate::core::lib::rsx::component::Component>]
        }
    };
}
