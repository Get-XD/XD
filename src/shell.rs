use crate::components::{app_icon, app_icon_sized};
use crate::data::*;
use crate::icons::icon;
use crate::pages::*;
use crate::state::*;
use crate::theme::Theme;
use crate::ui::*;
use gpui::{App, Context, Div, FontWeight, Hsla, InteractiveElement, IntoElement, KeyDownEvent, ParentElement, Render, ScrollHandle, SharedString, Stateful, StatefulInteractiveElement, Styled, Window, WindowControlArea, div, px};
use gpui::prelude::{FluentBuilder};

pub struct StoreShell {
    store: StoreHandle,
    scroll: ScrollHandle,
    scroll_route: Route,
}

impl StoreShell {
    pub fn new(store: StoreHandle) -> Self {
        Self {
            store,
            scroll: ScrollHandle::new(),
            scroll_route: Route::Home,
        }
    }
}

impl Render for StoreShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let store = self.store.clone();
        let state = store.read(cx);
        let theme = state.theme.clone();
        let collapsed = state.sidebar_collapsed;
        let route = state.route;
        let updates = state.updates();
        let active = state.active_downloads();
        let dialog = state.dialog.clone();
        let confirmation = state.confirmation.clone();
        let keep_data = state.keep_data;
        let prefs = state.preferences.clone();
        let page_title = route.title().to_string();
        let toasts = state.toasts.clone();
        let command_query = state.command_query.clone();
        let apps = state.apps.clone();
        let active_input = state.active_input;

        if self.scroll_route != route {
            self.scroll = ScrollHandle::new();
            self.scroll_route = route;
        }

        let sidebar_w = if collapsed { 74.0 } else { 230.0 };
        let content_left = sidebar_w;

        // keyboard handling
        let store_key = store.clone();
        let root = div()
            .id("store-shell")
            .size_full()
            .relative()
            .bg(theme.background)
            .text_color(theme.foreground)
            .font_family("Manrope")
            .text_size(px(13.0))
            .on_key_down(move |event: &KeyDownEvent, _, cx| {
                let key = event.keystroke.key.as_str();
                let mod_k = (event.keystroke.modifiers.control || event.keystroke.modifiers.platform)
                    && key.eq_ignore_ascii_case("k");
                if mod_k {
                    store_key.update(cx, |s, _| {
                        if s.dialog == DialogKind::CommandPalette {
                            s.dialog = DialogKind::None;
                            s.blur_input();
                        } else {
                            s.dialog = DialogKind::CommandPalette;
                            s.focus_input(ActiveInput::CommandSearch);
                        }
                    });
                    cx.notify(store_key.entity_id());
                    return;
                }
                if key == "escape" {
                    store_key.update(cx, |s, _| {
                        s.dialog = DialogKind::None;
                        s.confirmation = None;
                        s.blur_input();
                    });
                    cx.notify(store_key.entity_id());
                    return;
                }
                if key == "backspace" {
                    store_key.update(cx, |s, _| {
                        s.backspace();
                    });
                    cx.notify(store_key.entity_id());
                    return;
                }
                if key.len() == 1 && !event.keystroke.modifiers.control && !event.keystroke.modifiers.platform {
                    let ch = key.chars().next().unwrap();
                    let has_focus = store_key.read(cx).active_input != ActiveInput::None;
                    if has_focus {
                        store_key.update(cx, |s, _| {
                            s.type_char(ch);
                        });
                        cx.notify(store_key.entity_id());
                    }
                }
            })
            .child(sidebar(theme.clone(), store.clone(), collapsed, route, updates.len(), active, cx))
            .child(
                div()
                    .id("main-col")
                    .ml(px(content_left))
                    .flex()
                    .flex_col()
                    .h_full()
                    .child(header(
                        theme.clone(),
                        store.clone(),
                        page_title,
                        window.is_maximized(),
                        cx,
                    ))
                    .child(
                        div()
                            .id("main-scroll")
                            .flex_1()
                            .min_h(px(0.0))
                            .overflow_hidden()
                            .overflow_y_scroll()
                            .track_scroll(&self.scroll)
                            .child(
                                div()
                                    .min_h_full()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .id("page-content")
                                            .flex_1()
                                            .px(px(36.0))
                                            .pt(px(theme.content_pad_top()))
                                            .pb(px(32.0))
                                            .max_w(px(1540.0))
                                            .w_full()
                                            .overflow_hidden()
                                            .child(page_body(theme.clone(), store.clone(), cx)),
                                    )
                                    .child(footer(theme.clone())),
                            )
                    ),
            )
            .when(dialog == DialogKind::CommandPalette, |d| {
                d.child(command_palette(
                    theme.clone(),
                    store.clone(),
                    &command_query,
                    &apps,
                    cx,
                ))
            })
            .when(dialog == DialogKind::Notifications, |d| {
                d.child(notifications_dialog(theme.clone(), store.clone(), updates.len(), cx))
            })
            .when(dialog == DialogKind::Confirmation, |d| {
                d.child(confirmation_dialog(
                    theme.clone(),
                    store.clone(),
                    confirmation.as_ref(),
                    keep_data,
                    &prefs,
                    &updates,
                    cx,
                ))
            })
            .child(toasts_layer(&theme, &toasts));

        // track focus for inputs that open palette
        let _ = active_input;
        root
    }
}

fn caption_buttons(theme: Theme, maximized: bool) -> Stateful<Div> {
    let mut hover_bg = theme.muted_fg;
    hover_bg.a = 0.12;
    let close_red = Hsla {
        h: 0.986,
        s: 0.864,
        l: 0.488,
        a: 1.0,
    };
    let white = Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 1.0,
    };

    let mut glyph = icon("square").size(px(14.0)).text_color(theme.muted_fg);
    if maximized {
        glyph = icon("copy").size(px(14.0)).text_color(theme.muted_fg);
    }

    div()
        .id("caption-buttons")
        .flex()
        .flex_shrink_0()
        .items_center()
        .h_full()
        .mr(px(-36.0))
        .child(
            div()
                .id("tb-min")
                .w(px(46.0))
                .h(px(46.0))
                .flex_shrink_0()
                .flex()
                .items_center()
                .justify_center()
                .hover(|s| s.bg(hover_bg))
                .window_control_area(WindowControlArea::Min)
                .child(icon("minus").size(px(14.0)).text_color(theme.muted_fg)),
        )
        .child(
            div()
                .id("tb-max")
                .w(px(46.0))
                .h(px(46.0))
                .flex_shrink_0()
                .flex()
                .items_center()
                .justify_center()
                .hover(|s| s.bg(hover_bg))
                .window_control_area(WindowControlArea::Max)
                .child(glyph),
        )
        .child(
            div()
                .id("tb-close")
                .w(px(46.0))
                .h(px(46.0))
                .flex_shrink_0()
                .flex()
                .items_center()
                .justify_center()
                .hover(|s| s.bg(close_red))
                .window_control_area(WindowControlArea::Close)
                .child(
                    icon("x")
                        .id("tb-close-ico")
                        .size(px(14.0))
                        .text_color(theme.muted_fg)
                        .hover(|s| s.text_color(white)),
                ),
        )
}

fn page_body(theme: Theme, store: StoreHandle, cx: &mut App) -> Div {
    let route = store.read(cx).route;
    match route {
        Route::Home => home_page(theme, store, cx),
        Route::Explore => explore_page(theme, store, cx),
        Route::Installed => installed_page(theme, store, cx),
        Route::Updates => updates_page(theme, store, cx),
        Route::Downloads => downloads_page(theme, store, cx),
        Route::Details => details_page(theme, store, cx),
        Route::Settings => settings_page(theme, store, cx),
        Route::About => about_page(theme, store, cx),
    }
}

fn sidebar(
    theme: Theme,
    store: StoreHandle,
    collapsed: bool,
    route: Route,
    updates_count: usize,
    active: usize,
    cx: &mut App,
) -> Stateful<Div> {
    let w = if collapsed { 74.0 } else { 230.0 };
    let mut nav = div().flex().flex_col().gap(px(7.0));

    for (r, label, icon_name) in Route::nav_items() {
        let active_nav = route == r;
        let store_c = store.clone();
        let mut item = div()
            .id(SharedString::from(format!("nav-{}", r.path())))
            .relative()
            .flex()
            .items_center()
            .gap(px(13.0))
            .h(px(44.0))
            .w_full()
            .px(if collapsed { px(0.0) } else { px(15.0) })
            .when(collapsed, |d| d.justify_center())
            .rounded(px(7.0))
            .text_size(px(12.0))
            .text_color(if active_nav {
                theme.primary
            } else {
                theme.muted_fg
            })
            .cursor_pointer()
            .when(active_nav, |d| d.bg(theme.blue_soft));

        if active_nav {
            item = item.child(
                div()
                    .absolute()
                    .left(if collapsed { px(0.0) } else { px(-15.0) })
                    .h(px(21.0))
                    .w(px(3.0))
                    .rounded_tr(px(3.0))
                    .rounded_br(px(3.0))
                    .bg(theme.primary),
            );
        }

        item = item.child(
            icon(icon_name)
                .size(px(18.0))
                .text_color(if active_nav {
                    theme.primary
                } else {
                    theme.muted_fg
                }),
        );
        if !collapsed {
            item = item.child(div().child(label.to_string()));
        }
        if r == Route::Updates && updates_count > 0 {
            let badge = div()
                .absolute()
                .right(if collapsed { px(0.0) } else { px(15.0) })
                .top(if collapsed { px(0.0) } else { px(12.5) })
                .flex()
                .items_center()
                .justify_center()
                .w(px(if collapsed { 13.0 } else { 20.0 }))
                .h(px(if collapsed { 13.0 } else { 19.0 }))
                .rounded(px(5.0))
                .bg(theme.primary)
                .text_color(theme.primary_fg)
                .text_size(px(if collapsed { 7.0 } else { 10.0 }))
                .child(updates_count.to_string());
            item = item.child(badge);
        }
        if r == Route::Downloads && active > 0 {
            let badge = div()
                .absolute()
                .right(if collapsed { px(0.0) } else { px(15.0) })
                .top(if collapsed { px(0.0) } else { px(12.5) })
                .flex()
                .items_center()
                .justify_center()
                .w(px(if collapsed { 13.0 } else { 20.0 }))
                .h(px(if collapsed { 13.0 } else { 19.0 }))
                .rounded(px(5.0))
                .bg(theme.secondary)
                .text_color(theme.muted_fg)
                .text_size(px(if collapsed { 7.0 } else { 10.0 }))
                .child(active.to_string());
            item = item.child(badge);
        }

        item = item.on_click(move |_, _, cx| {
            store_c.update(cx, |s, _| {
                s.navigate(r, None);
                s.blur_input();
            });
            cx.notify(store_c.entity_id());
        });
        nav = nav.child(item);
    }

    // bottom section
    let mut bottom = div().mt_auto();

    if !collapsed {
        bottom = bottom.child(
            div()
                .flex()
                .items_center()
                .px(px(8.0))
                .py(px(18.0))
                .text_size(px(8.0))
                .text_color(theme.muted_fg)
                .whitespace_nowrap()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .child(
                            div()
                                .size(px(5.0))
                                .rounded_full()
                                .bg(theme.success)
                                .mr(px(7.0)),
                        )
                        .child("A little software. A lot of possibility.".to_string()),
                ),
        );
    }

    for (r, label, icon_name) in [
        (Route::Settings, "Settings", "settings"),
        (Route::About, "About", "info"),
    ] {
        let active_nav = route == r;
        let store_c = store.clone();
        let mut item = div()
            .id(SharedString::from(format!("nav-{}", r.path())))
            .relative()
            .flex()
            .items_center()
            .gap(px(13.0))
            .h(px(44.0))
            .w_full()
            .px(if collapsed { px(0.0) } else { px(15.0) })
            .when(collapsed, |d| d.justify_center())
            .rounded(px(7.0))
            .text_size(px(12.0))
            .text_color(if active_nav {
                theme.primary
            } else {
                theme.muted_fg
            })
            .cursor_pointer()
            .when(active_nav, |d| d.bg(theme.blue_soft));
        if active_nav {
            item = item.child(
                div()
                    .absolute()
                    .left(if collapsed { px(0.0) } else { px(-15.0) })
                    .h(px(21.0))
                    .w(px(3.0))
                    .rounded_tr(px(3.0))
                    .rounded_br(px(3.0))
                    .bg(theme.primary),
            );
        }
        item = item.child(
            icon(icon_name)
                .size(px(18.0))
                .text_color(if active_nav {
                    theme.primary
                } else {
                    theme.muted_fg
                }),
        );
        if !collapsed {
            item = item.child(label.to_string());
        }
        item = item.on_click(move |_, _, cx| {
            store_c.update(cx, |s, _| {
                s.navigate(r, None);
            });
            cx.notify(store_c.entity_id());
        });
        bottom = bottom.child(item);
    }

    let store_ver = store.clone();
    let store_col = store.clone();
    let version_row = div()
        .flex()
        .items_center()
        .justify_between()
        .mt(px(17.0))
        .mx(px(8.0))
        .pt(px(10.0))
        .border_t(px(1.0))
        .border_color(theme.border)
        .text_size(px(10.0))
        .text_color(theme.muted_fg)
        .when(collapsed, |d| d.justify_center().mx(px(0.0)))
        .child(
            div()
                .when(!collapsed, |d| {
                    d.child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child("XD".to_string())
                            .child(
                                div().text_color(theme.foreground).child("v1.0.0".to_string()),
                            ),
                    )
                }),
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .size(px(28.0))
                .rounded_md()
                .text_color(theme.muted_fg)
                .hover(|s| s.bg(theme.accent))
                .child(
                    icon(if collapsed {
                        "panel-left-open"
                    } else {
                        "panel-left-close"
                    })
                    .size(px(16.0))
                    .text_color(theme.muted_fg),
                )
                .id("sidebar-toggle").on_click(move |_, _, cx| {
                    store_col.update(cx, |s, _| {
                        s.sidebar_collapsed = !s.sidebar_collapsed;
                    });
                    cx.notify(store_col.entity_id());
                }),
        );
    let _ = store_ver;
    bottom = bottom.child(version_row);

    div()
        .id("sidebar")
        .absolute()
        .left(px(0.0))
        .top(px(0.0))
        .bottom(px(0.0))
        .w(px(w))
        .flex()
        .flex_col()
        .px(px(if collapsed { 10.0 } else { 15.0 }))
        .pt(px(28.0))
        .pb(px(12.0))
        .bg(theme.sidebar)
        .border_r(px(1.0))
        .border_color(theme.border)
        
        .child(brand(theme.clone(), collapsed, store.clone(), cx))
        .when(!collapsed, |d| {
            d.child(
                div()
                    .px(px(14.0))
                    .mb(px(12.0))
                    .text_size(px(9.0))
                    
                    .text_color(theme.muted_fg)
                    .child("WORKSPACE".to_string()),
            )
        })
        .child(nav)
        .child(bottom)
}

fn brand(theme: Theme, collapsed: bool, store: StoreHandle, _cx: &mut App) -> Stateful<Div> {
    let store_c = store.clone();
    div()
        .flex()
        .items_center()
        .gap(px(11.0))
        .px(if collapsed { px(8.0) } else { px(10.0) })
        .mb(px(43.0))
        .cursor_pointer()
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .w(px(38.0))
                .h(px(40.0))
                .rounded(px(10.0))
                .bg(theme.primary)
                .text_color(theme.primary_fg)
                .shadow_md()
                .child(icon("package-open").size(px(23.0)).text_color(theme.primary_fg)),
        )
        .when(!collapsed, |d| {
            d.child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(17.0))
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(theme.foreground)
                            .child("XD".to_string()),
                    )
                    .child(
                        div()
                            .mt(px(5.0))
                            .text_size(px(7.0))
                            .line_height(px(9.0))
                            
                            .text_color(theme.muted_fg)
                            .flex()
                            .flex_col()
                            .gap(px(1.0))
                            .child("YOUR PERSONAL SOFTWARE".to_string())
                            .child("HUB".to_string()),
                    ),
            )
        })
        .id("brand").on_click(move |_, _, cx| {
            store_c.update(cx, |s, _| {
                s.navigate(Route::Home, None);
            });
            cx.notify(store_c.entity_id());
        })
}

fn header(
    theme: Theme,
    store: StoreHandle,
    title: String,
    maximized: bool,
    cx: &mut App,
) -> Stateful<Div> {
    let store_search = store.clone();
    let store_notif = store.clone();
    let store_settings = store.clone();
    let query = store.read(cx).command_query.clone();
    let focused = matches!(
        store.read(cx).active_input,
        ActiveInput::HeaderSearch | ActiveInput::CommandSearch
    );

    div()
        .id("header")
        .flex()
        .items_center()
        .gap(px(14.0))
        .h(px(73.0))
        .px(px(36.0))
        .border_b(px(1.0))
        .border_color(theme.border)
        .flex_shrink_0()
        .child(
            div()
                .id("header-drag")
                .flex()
                .items_center()
                .flex_1()
                .min_w(px(0.0))
                .h_full()
                .window_control_area(WindowControlArea::Drag)
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.foreground)
                        .child(title),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(14.0))
                .text_color(theme.muted_fg)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .w(px(298.0))
                        .h(px(34.0))
                        .px(px(10.0))
                        .rounded(px(6.0))
                        .border_1()
                        .border_color(if focused { theme.primary } else { theme.border })
                        .bg(theme.card)
                        .cursor_text()
                        .child(icon("search").size(px(17.0)).text_color(theme.muted_fg))
                        .child(
                            div()
                                .flex_1()
                                .min_w(px(0.0))
                                .text_size(px(10.0))
                                .text_color(if query.is_empty() {
                                    theme.muted_fg
                                } else {
                                    theme.foreground
                                })
                                .child(if query.is_empty() {
                                    "Search applications...".to_string()
                                } else {
                                    query.clone()
                                }),
                        )
                        .child(
                            div()
                                .px(px(4.0))
                                .py(px(2.0))
                                .border_1()
                                .border_color(theme.border)
                                .rounded(px(3.0))
                                .text_size(px(9.0))
                                .text_color(theme.muted_fg)
                                .child("Ctrl K".to_string()),
                        )
                        .id("header-search").on_click(move |_, _, cx| {
                            store_search.update(cx, |s, _| {
                                s.focus_input(ActiveInput::HeaderSearch);
                                s.dialog = DialogKind::CommandPalette;
                            });
                            cx.notify(store_search.entity_id());
                        }),
                )
                .child(
                    div()
                        .relative()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(36.0))
                        .rounded_md()
                        .text_color(theme.muted_fg)
                        .hover(|s| s.bg(theme.accent))
                        .child(icon("bell").size(px(16.0)).text_color(theme.muted_fg))
                        .child(
                            div()
                                .absolute()
                                .right(px(9.0))
                                .top(px(7.0))
                                .size(px(4.0))
                                .rounded_full()
                                .bg(theme.primary),
                        )
                        .id("bell").on_click(move |_, _, cx| {
                            store_notif.update(cx, |s, _| {
                                s.dialog = if s.dialog == DialogKind::Notifications {
                                    DialogKind::None
                                } else {
                                    DialogKind::Notifications
                                };
                            });
                            cx.notify(store_notif.entity_id());
                        }),
                )
                .child(
                    div()
                        .w(px(1.0))
                        .h(px(18.0))
                        .bg(theme.border),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(36.0))
                        .rounded_md()
                        .text_color(theme.muted_fg)
                        .hover(|s| s.bg(theme.accent))
                        .child(icon("settings").size(px(16.0)).text_color(theme.muted_fg))
                        .id("header-settings").on_click(move |_, _, cx| {
                            store_settings.update(cx, |s, _| {
                                s.navigate(Route::Settings, None);
                            });
                            cx.notify(store_settings.entity_id());
                        }),
                ),
        )
        .child(caption_buttons(theme.clone(), maximized))
}

fn footer(theme: Theme) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(15.0))
        .mx(px(36.0))
        .pt(px(17.0))
        .pb(px(17.0))
        .border_t(px(1.0))
        .border_color(theme.border)
        .text_size(px(9.0))
        .text_color(theme.muted_fg)
        .flex_shrink_0()
        .child(
            div()
                .flex()
                .items_center()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .child(
                            div()
                                .size(px(5.0))
                                .rounded_full()
                                .bg(theme.success)
                                .mr(px(7.0)),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .child("Local prototype".to_string())
                                .child(
                                    div().mx(px(8.0)).child("·".to_string()),
                                )
                                .child("Windows 10 / 11".to_string()),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .child("Made with care by XD".to_string())
                .child(div().mx(px(8.0)).child("·".to_string()))
                .child("v1.0.0".to_string()),
        )
}

fn command_palette(
    theme: Theme,
    store: StoreHandle,
    query: &str,
    apps: &[StoreApp],
    _cx: &mut App,
) -> Stateful<Div> {
    let ql = query.to_lowercase();
    let filtered_apps: Vec<StoreApp> = apps
        .iter()
        .filter(|a| format!("{} {}", a.name, a.description).to_lowercase().contains(&ql))
        .cloned()
        .collect();

    let mut results = div().flex().flex_col().max_h(px(300.0)).overflow_hidden();

    for app in filtered_apps.iter().take(8) {
        let store_c = store.clone();
        let id = app.id.clone();
        results = results.child(
            div()
                .flex()
                .items_center()
                .gap(px(13.0))
                .min_h(px(46.0))
                .px(px(8.0))
                .py(px(8.0))
                .rounded_md()
                .text_size(px(12.0))
                .text_color(theme.foreground)
                .hover(|s| s.bg(theme.accent))
                .cursor_pointer()
                .child(app_icon_sized(&theme, app, 30.0))
                .child(
                    div()
                        .flex_1()
                        .child(app.name.clone())
                        .child(
                            div()
                                .mt(px(3.0))
                                .text_size(px(9.0))
                                .text_color(theme.muted_fg)
                                .child(app.category.label().to_string()),
                        ),
                )
                .child(
                    icon("arrow-right")
                        .size(px(14.0))
                        .text_color(theme.muted_fg),
                )
                .id(SharedString::from(format!("cmdapp-{}", id))).on_click(move |_, _, cx| {
                    store_c.update(cx, |s, _| {
                        s.dialog = DialogKind::None;
                        s.command_query.clear();
                        s.navigate(Route::Details, Some(id.clone()));
                        s.blur_input();
                    });
                    cx.notify(store_c.entity_id());
                }),
        );
    }

    for (r, label, icon_name) in Route::nav_items() {
        if !label.to_lowercase().contains(&ql) {
            continue;
        }
        let store_c = store.clone();
        results = results.child(
            div()
                .flex()
                .items_center()
                .gap(px(13.0))
                .min_h(px(46.0))
                .px(px(8.0))
                .py(px(8.0))
                .rounded_md()
                .text_size(px(12.0))
                .text_color(theme.foreground)
                .hover(|s| s.bg(theme.accent))
                .cursor_pointer()
                .child(icon(icon_name).size(px(18.0)).text_color(theme.muted_fg))
                .child(format!("Go to {label}"))
                .child(
                    div()
                        .ml_auto()
                        .child(icon("arrow-right").size(px(14.0)).text_color(theme.muted_fg)),
                )
                .id(SharedString::from(format!("cmdnav-{}", label))).on_click(move |_, _, cx| {
                    store_c.update(cx, |s, _| {
                        s.dialog = DialogKind::None;
                        s.navigate(r, None);
                        s.blur_input();
                    });
                    cx.notify(store_c.entity_id());
                }),
        );
    }

    let store_close = store.clone();
    let store_input = store.clone();
    let q = query.to_string();

    modal(
        &theme,
        "Search & commands",
        Some("Applications and workspace"),
        520.0,
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .min_h(px(39.0))
                    .px(px(12.0))
                    .rounded_md()
                    .border_1()
                    .border_color(theme.primary)
                    .bg(theme.card)
                    .child(icon("search").size(px(17.0)).text_color(theme.muted_fg))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(12.0))
                            .text_color(if q.is_empty() {
                                theme.muted_fg
                            } else {
                                theme.foreground
                            })
                            .child(if q.is_empty() {
                                "Search apps or jump to a page...".to_string()
                            } else {
                                q.clone()
                            }),
                    )
                    .child(
                        div()
                            .px(px(4.0))
                            .py(px(2.0))
                            .border_1()
                            .border_color(theme.border)
                            .rounded(px(3.0))
                            .text_size(px(9.0))
                            .text_color(theme.muted_fg)
                            .child("ESC".to_string()),
                    )
                    .id("cmd-esc").on_click(move |_, _, cx| {
                        store_input.update(cx, |s, _| {
                            s.focus_input(ActiveInput::CommandSearch);
                        });
                        cx.notify(store_input.entity_id());
                    }),
            )
            .child(results),
        None,
    )
    .id("cmd-palette-overlay").on_click(move |event, _, cx| {
        // stop background click from only closing - actually close when clicking overlay
        let _ = event;
        store_close.update(cx, |s, _| {
            if s.dialog == DialogKind::CommandPalette {
                s.dialog = DialogKind::None;
                s.blur_input();
            }
        });
        cx.notify(store_close.entity_id());
    })
}

fn notifications_dialog(theme: Theme, store: StoreHandle, updates_count: usize, _cx: &mut App) -> Stateful<Div> {
    let items = vec![
        (
            "refresh-cw",
            format!("{updates_count} software updates available"),
            "Keep your favorite tools at their best.".to_string(),
        ),
        (
            "check-circle-2",
            "SkyBox installation completed — demo".to_string(),
            "Version 1.8.2 is ready in your library.".to_string(),
        ),
        (
            "circle-alert",
            "FocusFlow download failed — demo".to_string(),
            "Review the static example in Downloads.".to_string(),
        ),
        (
            "package-open",
            "XD is up to date".to_string(),
            "You are using the v1.0.0 prototype.".to_string(),
        ),
    ];

    let mut list = div().flex().flex_col();
    for (ic, title, text) in items {
        list = list.child(
            div()
                .flex()
                .items_start()
                .gap(px(15.0))
                .pt(px(18.0))
                .pb(px(18.0))
                .border_t(px(1.0))
                .border_color(theme.border)
                .child(icon(ic).size(px(19.0)).text_color(theme.primary))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(5.0))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.foreground)
                                .child(title),
                        )
                        .child(
                            div()
                                .text_size(px(10.0))
                                .line_height(px(17.0))
                                .text_color(theme.muted_fg)
                                .child(text),
                        ),
                ),
        );
    }

    let store_c = store.clone();
    modal(
        &theme,
        "Notifications",
        Some("Your recent store activity"),
        480.0,
        list,
        None,
    )
    .id("notif-overlay").on_click(move |_, _, cx| {
        store_c.update(cx, |s, _| {
            if s.dialog == DialogKind::Notifications {
                s.dialog = DialogKind::None;
            }
        });
        cx.notify(store_c.entity_id());
    })
}

fn confirmation_dialog(
    theme: Theme,
    store: StoreHandle,
    confirmation: Option<&Confirmation>,
    keep_data: bool,
    prefs: &Preferences,
    updates: &[StoreApp],
    _cx: &mut App,
) -> Div {
    let Some(conf) = confirmation else {
        return div();
    };

    let title = match conf.action {
        ConfirmationAction::UpdateAll => "Update all applications".to_string(),
        ConfirmationAction::Uninstall => format!("Uninstall {}", conf.app.as_ref().map(|a| a.name.as_str()).unwrap_or("")),
        ConfirmationAction::Update => format!("Update {}", conf.app.as_ref().map(|a| a.name.as_str()).unwrap_or("")),
        ConfirmationAction::Install => format!("Install {}", conf.app.as_ref().map(|a| a.name.as_str()).unwrap_or("")),
    };

    let mut body = div().flex().flex_col().gap(px(16.0));

    if let Some(app) = &conf.app {
        body = body.child(
            div()
                .flex()
                .items_center()
                .gap(px(15.0))
                .py(px(15.0))
                .child(app_icon(&theme, app, false))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(5.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.foreground)
                                .child(app.name.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(theme.muted_fg)
                                .child(match conf.action {
                                    ConfirmationAction::Update => {
                                        format!("{} → v{}", app.installed.clone().unwrap_or_default(), app.version)
                                    }
                                    _ => format!("v{}", app.version),
                                }),
                        ),
                ),
        );

        if conf.action == ConfirmationAction::Uninstall {
            let store_c = store.clone();
            body = body.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .mb(px(4.0))
                    .text_size(px(12.0))
                    .text_color(theme.foreground)
                    .cursor_pointer()
                    .child(
                        div()
                            .size(px(14.0))
                            .rounded(px(3.0))
                            .border_1()
                            .border_color(theme.border)
                            .bg(if keep_data { theme.primary } else { theme.card })
                            .flex()
                            .items_center()
                            .justify_center()
                            .when(keep_data, |d| {
                                d.child(icon("check").size(px(10.0)).text_color(theme.primary_fg))
                            }),
                    )
                    .child("Keep application data".to_string())
                    .id("keep-data").on_click(move |_, _, cx| {
                        store_c.update(cx, |s, _| {
                            s.keep_data = !s.keep_data;
                        });
                        cx.notify(store_c.entity_id());
                    }),
            );
        } else {
            let mut facts = div().flex().flex_col().gap(px(19.0));
            for (label, value) in [
                (
                    "Installation location",
                    format!("{}\\{}", prefs.install_directory, app.name),
                ),
                ("Required disk space", format!("{} MB", app.size)),
            ] {
                facts = facts.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(5.0))
                        .child(
                            div()
                                .text_size(px(10.0))
                                .text_color(theme.muted_fg)
                                .child(label.to_string()),
                        )
                        .child(div().text_size(px(11.0)).text_color(theme.foreground).child(value)),
                );
            }
            body = body.child(facts);
        }
    }

    if conf.action == ConfirmationAction::UpdateAll {
        body = body.child(
            div()
                .text_size(px(12.0))
                .text_color(theme.muted_fg)
                .child(format!(
                    "{} applications will be marked up to date.",
                    updates.len()
                )),
        );
    }

    let store_cancel = store.clone();
    let store_confirm = store.clone();
    let footer = div()
        .flex()
        .items_center()
        .justify_end()
        .gap(px(10.0))
        .child(
            button(
                &theme,
                ButtonVariant::Outline,
                ButtonSize::Default,
                Some("Cancel"),
                None,
                false,
                move |_, _, cx| {
                    store_cancel.update(cx, |s, _| {
                        s.dialog = DialogKind::None;
                        s.confirmation = None;
                    });
                    cx.notify(store_cancel.entity_id());
                },
            ),
        )
        .child({
            let label = match conf.action {
                ConfirmationAction::UpdateAll => "Simulate all updates",
                ConfirmationAction::Uninstall => "Simulate uninstall",
                ConfirmationAction::Update => "Simulate update",
                ConfirmationAction::Install => "Simulate install",
            };
            let variant = if conf.action == ConfirmationAction::Uninstall {
                ButtonVariant::Destructive
            } else {
                ButtonVariant::Default
            };
            button(
                &theme,
                variant,
                ButtonSize::Default,
                Some(label),
                None,
                false,
                move |_, _, cx| {
                    store_confirm.update(cx, |s, _| {
                        match s.confirmation.clone() {
                            Some(c) if c.action == ConfirmationAction::UpdateAll => {
                                let n = s.updates().len();
                                let ids: Vec<String> = s.updates().iter().map(|a| a.id.clone()).collect();
                                for id in ids {
                                    s.begin_download(&id);
                                }
                                s.toast(
                                    format!("{n} updates queued for download"),
                                    Some("Track progress on the Downloads page.".into()),
                                    ToastKind::Success,
                                );
                            }
                            Some(c) => {
                                if let Some(app) = &c.app {
                                    match c.action {
                                        ConfirmationAction::Uninstall => {
                                            s.dispatch(StoreAction::Uninstall(app.id.clone()));
                                            let desc = if s.keep_data {
                                                "Application data preference: keep."
                                            } else {
                                                "Application data preference: remove."
                                            };
                                            s.toast(
                                                format!("{}: uninstall simulated", app.name),
                                                Some(desc.to_string()),
                                                ToastKind::Success,
                                            );
                                        }
                                        ConfirmationAction::Update => {
                                            s.begin_download(&app.id);
                                            s.toast(
                                                format!("{}: update queued", app.name),
                                                Some("Track progress on the Downloads page.".into()),
                                                ToastKind::Success,
                                            );
                                        }
                                        _ => {
                                            s.begin_download(&app.id);
                                            s.toast(
                                                format!("{}: download started", app.name),
                                                Some("Track progress on the Downloads page.".into()),
                                                ToastKind::Success,
                                            );
                                        }
                                    }
                                }
                            }
                            None => {}
                        }
                        s.dialog = DialogKind::None;
                        s.confirmation = None;
                    });
                    cx.notify(store_confirm.entity_id());
                },
            )
        });

    modal(&theme, &title, Some("This changes local demonstration state only. No software on your device will be modified."), 460.0, body, Some(footer))
}
