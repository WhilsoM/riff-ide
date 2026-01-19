use crate::core::stores::global_store::global_store;
use crate::core::types::types::Element;
use crate::core::ui::ui_kit::style::{Align, Display, FlexDirection, Justify};
use crate::core::ui::ui_kit::{Button, Color, Style, StyleSheet, Text, View};
use crate::modules::editor::components::TopPanel;
use crate::modules::editor::stores::theme_store;
use crate::rsx;
use riff_rsx_macro::component;

#[component]
pub fn Navbar(ctx: eframe::egui::Context) -> Element {
    let global_store = global_store();
    let is_show_settings = global_store.is_show_settings.clone();

    let theme = theme_store();

    // TODO: FIXME: Ебучий навбар тянется куда то дохуя далеко, + кнопка внутри не встает в конец.
    // Я мб думаю что проблема в кнопке внутри. Не хочу сейчас этим заниматься, я работал 9 часов я дико заебался
    //
    // Button {
    //     text: "Settings".to_string(),
    //     on_click: Some(settings_handler),
    // }
    //
    // 1. создать состояние для показа
    // 2. при клике менять состояние
    // 3. если true то отображать ui

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
    let s = StyleSheet::new().with(
        "settings_panel",
        Style::new().flex_direction(FlexDirection::Column),
    );
    //if unsafe { MOUSE_ENABLED } {
    //    println!("MOUSE ENABLED")
    //} else {
    //    println!("MOUSE DISABLED");
    //}
    //println!("IS SHOW SETTINGS: {}", is_show_settings.get(&ctx));

    if is_show_settings.get(&ctx) {
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
           // 1. нужно в колонку перенести элементы
           // 2. добавить им задний фон
           // 3. создать страницу settings
           // 4. добавить там показ размера шрифта
           // 5. добавить всплыл окно когда меняешь размер шрифта (показывать в цифрах)
                                           View {
                                               style: s.get("settings_panel"),
                                               children: {
                                                Button {
                                                   text: "⚙ Settings".to_string(),
                                                   on_click: {
                                                       let ctx = ctx.clone();

                                                       Some(std::rc::Rc::new(move || {
                                                           println!("Settings clicked");
                                                           global_store.change_show_settings(&ctx);
                                                       }))
                                                   },
                                           };
                                           View {
                                               children: {
                                                   Text {
                                                       content: "Hello".to_string(),
                                                   }
                                               }
                                           };

                                               }
                                           };
                                           View {
                                               style: s.get("settings_panel"),
                                               children: {
                                                Button {
                                                   text: "⚙ Hello".to_string(),
                                                   on_click: {
                                                       //let ctx = ctx.clone();

                                                       Some(std::rc::Rc::new(move || {
                                                           println!("hell clicked");
                                                       }))
                                                   },
                                           };
                                           View {
                                               children: {
                                                   Text {
                                                       content: "Hello".to_string(),
                                                   }
                                               }
                                           };

                                               }
                                           };


                                      }
                                   }
                               }
                           }
        }
    } else {
        rsx! {
            TopPanel {
                id: "navbar".to_string(),
                children: {
                    View {
                        style: navbar_style.get("navbar"),
                        children: {
                            Button {
                                text: "⚙ Settings".to_string(),
                                        on_click: {
            let ctx = ctx.clone();

            Some(std::rc::Rc::new(move || {
                println!("Settings clicked");
                global_store.change_show_settings(&ctx);
            }))
        },


                            };
                             Button {
                                text: "⚙ Hello mybo".to_string(),
                                        on_click: {

            Some(std::rc::Rc::new(move || {
                println!("Settings clicked");
            }))
        },


                            };
                       }
                    }
                }
            }
        }
    }
}
