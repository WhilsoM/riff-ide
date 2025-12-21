use crate::core::lib::rsx::component::Element;
use crate::core::ui::ui_kit::button::ButtonStyle;
use crate::core::ui::ui_kit::style::{Align, Display, FlexDirection, Justify};
use crate::core::ui::ui_kit::{Button, Color, Style, StyleSheet, View};
use crate::modules::editor::components::TopPanel;
use crate::modules::editor::stores::theme_store;
use crate::rsx;
use riff_rsx_macro::component;
use std::rc::Rc;

#[component]
pub fn Navbar(ctx: eframe::egui::Context) -> Element {
    let theme = theme_store();
    let settings_handler = Rc::new(|| {
        println!("Settings clicked");
    });

    // TODO: FIXME: Ебучий навбар тянется куда то дохуя далеко, + кнопка внутри не встает в конец.
    // Я мб думаю что проблема в кнопке внутри. Не хочу сейчас этим заниматься, я работал 9 часов я дико заебался
    //
    // Button {
    //     text: "Settings".to_string(),
    //     on_click: Some(settings_handler),
    // }
    //
    let navbar_style = StyleSheet::new()
        .with(
            "navbar",
            Style::new()
                .padding_horizontal(10.0)
                .padding_vertical(7.0)
                .display(Display::Flex)
                .flex_direction(FlexDirection::Row)
                .flex(1)
                .background_color(theme.bg_main_200.get(&ctx))
                .align(Align::Center)
                .justify(Justify::End),
        )
        .with(
            "btn",
            Style::new().border_width(1.0).border_color(Color::RED),
        );

    rsx! {
        TopPanel {
            id: "navbar".to_string(),
            children: {
                View {
                    style: navbar_style.get("navbar"),
                    children: {
                        // TODO: Кнопка внутри не имеет style пропса с типом Option<Rc<Style>> как у View.
                        // Надо привести абсолютно весь UI кит к этому типу, потому что это пиздец. У нас же везде StyleSheet::new()
                        // Это как раз поможет делать глобальные стили для компонентов и быстро добавлять все нужные стили к новым UI компонентам в ките
                        Button {
                            text: "Settings".to_string(),
                            on_click: Some(settings_handler),
                        }
                    }
                }
            }
        }
    }
}
