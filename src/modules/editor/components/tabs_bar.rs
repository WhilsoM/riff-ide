use std::rc::Rc;

use crate::core::lib::rsx::Children;
use crate::core::lib::rsx::component::Element;
use crate::core::ui::ui_kit::style::{Align, Display, FlexDirection, Justify};
use crate::core::ui::ui_kit::{Button, Style, StyleSheet, Text, View};
use crate::modules::editor::stores::{Tab, editor_interactions_store, theme_store};
use crate::rsx;
use riff_rsx_macro::component;

#[component]
fn render_tab(tab: &Tab, index: usize, ctx: eframe::egui::Context) -> Element {
    let editor_interactions = editor_interactions_store();
    let theme = theme_store();
    let tab_index = index;
    let tab_path = tab.path.clone();
    let tabs_ref = editor_interactions.tabs.borrow();
    let is_dirty = tabs_ref.get(index).map(|t| t.is_dirty).unwrap_or(false);
    drop(tabs_ref);

    let file_name = tab_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Untitled")
        .to_string();

    let is_active = editor_interactions.active_tab_index.get(&ctx) == Some(index);

    let click_handler = {
        let ctx = ctx.clone();
        Rc::new(move || {
            editor_interactions_store().set_active_tab(&ctx, tab_index);
        })
    };

    let close_handler = {
        let ctx = ctx.clone();
        Rc::new(move || {
            editor_interactions_store().close_tab(&ctx, tab_index);
        })
    };

    let tab_bg_color = if is_active {
        theme.bg_main_100.get(&ctx)
    } else {
        theme.bg_main_200.get(&ctx)
    };

    let s = StyleSheet::new()
        .with(
            "tab",
            Style::new()
                .padding_xy(6.0, 6.0)
                .background_color(tab_bg_color)
                .align(Align::Center)
                .justify(Justify::SpaceBetween)
                .width(100.0)
                .flex_direction(FlexDirection::Row)
                .border_width(1.0)
                .border_color(theme.bg_main_300.get(&ctx)),
        )
        .with("tab_text", Style::new().flex_direction(FlexDirection::Row))
        .with(
            "btn",
            Style::new()
                .background_color(tab_bg_color)
                .border_color(theme.border_primary.get(&ctx)),
        );

    let dirty_circle_style = Rc::new(
        Style::new()
            .width(8.0)
            .height(8.0)
            .border_radius(4.0)
            .background_color(theme.accent_primary.get(&ctx))
            .margin(4.0),
    );

    if is_dirty {
        rsx! {
            View {
                style: s.get("tab"),
                children: {
                    View {
                        align: "center".to_string(),
                        justify: "center".to_string(),
                        style: s.get("tab_text"),
                        children: {
                            Text {
                                content: file_name.clone(),
                            };
                            View {
                                style: Some(dirty_circle_style.clone()),
                            }
                        }
                    };
                    Button {
                        text: "×".to_string(),
                        on_click: Some(close_handler),
                        style: s.get("btn"),
                        enabled: true,
                    }
                }
            }
        }
    } else {
        rsx! {
            View {
                style: s.get("tab"),
                align: "center".to_string(),
                justify: "space-between".to_string(),
                children: {
                    View {
                        align: "center".to_string(),
                        justify: "flex-start".to_string(),
                        style: s.get("tab_text"),
                        children: {
                            Button {
                                text: file_name.clone(),
                                on_click: Some(click_handler),
                                enabled: true,
                            }
                        }
                    };
                    Button {
                        text: "×".to_string(),
                        on_click: Some(close_handler),
                        style: s.get("btn"),
                        enabled: true,
                    }
                }
            }
        }
    }
}

#[component]
pub fn TabsBar(ctx: eframe::egui::Context) -> Element {
    let theme = theme_store();
    let editor_interactions = editor_interactions_store();
    let tabs_vec = editor_interactions.tabs.get(&ctx).clone();

    let tabs_container_style = Style::new()
        .display(Display::Flex)
        .flex_direction(FlexDirection::Row)
        .background_color(theme.bg_main_200.get(&ctx))
        .height(20.0);

    let tab_components: Vec<Element> = tabs_vec
        .iter()
        .enumerate()
        .map(|(index, tab)| render_tab(tab, index, ctx.clone()))
        .collect();

    rsx! {
        View {
            align: "flex-start".to_string(),
            justify: "flex-start".to_string(),
            style: Some(Rc::new(tabs_container_style)),
            children: Children::Multiple(tab_components),
        }
    }
}
