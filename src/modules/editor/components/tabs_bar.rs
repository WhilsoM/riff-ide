use crate::core::lib::rsx::Children;
use crate::core::types::types::Element;
use crate::core::ui::ui_kit::style::{Align, Display, FlexDirection, Justify, Overflow};
use crate::core::ui::ui_kit::{Button, Style, StyleSheet, Text, View};
use crate::modules::editor::stores::{editor_interactions_store, theme_store, Tab};
use crate::{on_click, rsx};
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

    let ctx_click_handler = ctx.clone();
    let ctx_close_handler = ctx.clone();

    fn click_handler(ctx: egui::Context, tab_index: usize) {
        editor_interactions_store().set_active_tab(&ctx, tab_index);
    }

    fn close_handler(ctx: egui::Context, tab_index: usize) {
        editor_interactions_store().close_tab(&ctx, tab_index);
    }

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
        .with(
            "tab_text",
            Style::new()
                .flex_direction(FlexDirection::Row)
                .justify(Justify::Center)
                .align(Align::Center),
        )
        .with(
            "btn",
            Style::new()
                .background_color(tab_bg_color)
                .border_color(theme.border_primary.get(&ctx)),
        )
        .with(
            "dirty_circle",
            Style::new()
                .width(8.0)
                .height(8.0)
                .border_radius(4.0)
                .background_color(theme.accent_primary.get(&ctx))
                .margin(4.0),
        )
        .with(
            "center",
            Style::new().justify(Justify::Start).align(Align::Start),
        );

    if is_dirty {
        rsx! {
            View {
                style: s.get("tab"),
                children: {
                    View {
                        style: s.get("tab_text"),
                        children: {
                            Text {
                                content: file_name.clone(),
                            };
                            View {
                                style: s.get("dirty_circle"),
                            }
                        }
                    };
                    Button {
                        text: "×".to_string(),
                        on_click: Some(on_click!(close_handler, ctx_close_handler, tab_index)),
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
                children: {
                    View {
                        style: s.get("tab_text"),
                        children: {
                            Button {
                                text: file_name.clone(),
                                on_click: Some(on_click!(click_handler, ctx_click_handler, tab_index)),
                                enabled: true,
                            }
                        }
                    };
                    Button {
                        text: "×".to_string(),
                        on_click: Some(on_click!(close_handler, ctx, tab_index)),
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

    let s = StyleSheet::new().with(
        "tabs_container",
        Style::new()
            .display(Display::Flex)
            .justify(Justify::Start)
            .align(Align::Start)
            .flex_direction(FlexDirection::Row)
            .overflow(Overflow::Scroll)
            .background_color(theme.bg_main_200.get(&ctx))
            .height(20.0),
    );

    let tab_components = tabs_vec
        .iter()
        .enumerate()
        .map(|(index, tab)| render_tab(tab, index, ctx.clone()))
        .collect();

    rsx! {
        View {
            style: s.get("tabs_container"),
            children: Children::Multiple(tab_components),
        }
    }
}
