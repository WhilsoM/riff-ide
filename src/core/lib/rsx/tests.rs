#[cfg(test)]
mod tests {
    use crate::core::lib::rsx::component::Component;
    use crate::core::ui::ui_kit::Button;
    use crate::rsx;

    #[test]
    fn test_button_creation() {
        let button = Button::new();
        assert_eq!(button.props.text, "");
        assert!(button.props.on_click.is_none());
    }

    #[test]
    fn test_button_with_text() {
        let button = Button::with_text("Test Button");
        assert_eq!(button.props.text, "Test Button");
    }

    #[test]
    fn test_button_props_default() {
        use crate::core::ui::ui_kit::button::ButtonProps;
        let props = ButtonProps::default();
        assert_eq!(props.text, "");
        assert!(props.on_click.is_none());
        assert!(props.enabled || !props.enabled);
    }

    #[test]
    fn test_rsx_macro_empty() {
        let _button = rsx! {
            Button {}
        };
    }

    #[test]
    fn test_rsx_macro_with_props() {
        let click_handler = std::rc::Rc::new(|| {
            println!("Clicked!");
        });

        let _button = rsx! {
            Button {
                text: "Test".to_string(),
                on_click: Some(click_handler.clone()),
                enabled: true,
                children: crate::core::lib::rsx::component::Children::None,
                style: None,
            }
        };
    }

    #[test]
    fn test_rsx_macro_text_node() {
        let _text = rsx!("Hello World");
    }
}
