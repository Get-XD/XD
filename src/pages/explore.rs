use crate::components::*;
use crate::icons::icon;
use crate::data::*;
use crate::state::{ActiveInput, Route, StoreHandle};
use crate::theme::Theme;
use crate::ui::*;
use gpui::{App, Div, FontWeight, InteractiveElement, ParentElement, SharedString, StatefulInteractiveElement, Styled, div, px};

pub fn explore_page(theme: Theme, store: StoreHandle, cx: &mut App) -> Div {
    let (search, category, sort, list) = {
        let s = store.read(cx);
        (
            s.explore_search.clone(),
            s.explore_category.clone(),
            s.explore_sort.clone(),
            s.explore_list,
        )
    };
    let apps = filter_catalog(
        &store.read(cx).apps,
        &search,
        &category,
        &sort,
    );
    let focused = store.read(cx).active_input == ActiveInput::ExploreSearch;

    let store_sort = store.clone();
    let store_view = store.clone();
    let store_focus = store.clone();
    let store_clear = store.clone();
    let store_cat = store.clone();

    let mut cat_row = div()
        .flex()
        .flex_wrap()
        .gap(px(8.0))
        .pb(px(18.0))
        .border_b(px(1.0))
        .border_color(theme.border);
    for c in Category::all() {
        let selected = category == *c;
        let store_c = store_cat.clone();
        let label = c.to_string();
        let label_click = label.clone();
        cat_row = cat_row.child(
            button_inner(
                &theme,
                if selected {
                    ButtonVariant::Secondary
                } else {
                    ButtonVariant::Ghost
                },
                ButtonSize::Sm,
                Some(&label),
                None,
                false,
                Some(32.0),
                Some(11.0),
                move |_, _, cx| {
                    store_c.update(cx, |s, _| {
                        s.explore_category = label_click.clone();
                    });
                    cx.notify(store_c.entity_id());
                },
            ),
        );
    }

    div()
        .flex()
        .flex_col()
        .child(page_intro(
            &theme,
            "Discover your next essential.",
            "Purpose-built applications for a better desktop experience.",
            None,
        ))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(14.0))
                .mb(px(20.0))
                .child(text_field(
                    &theme,
                    &search,
                    "Search applications...",
                    None,
                    focused,
                    move |cx| {
                        store_focus.update(cx, |s, _| {
                            s.focus_input(ActiveInput::ExploreSearch);
                        });
                        cx.notify(store_focus.entity_id());
                    },
                ))
                .child(select(
                    &theme,
                    &sort,
                    &[
                        "Recently Updated",
                        "Alphabetical",
                        "Recently Added",
                    ],
                    Some(180.0),
                    move |v, cx| {
                        store_sort.update(cx, |s, _| {
                            s.explore_sort = v;
                        });
                        cx.notify(store_sort.entity_id());
                    },
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(2.0))
                        .p(px(2.0))
                        .border_1()
                        .border_color(theme.border)
                        .rounded(px(7.0))
                        .child({
                            let store_v = store_view.clone();
                            button_inner(
                                &theme,
                                if !list {
                                    ButtonVariant::Secondary
                                } else {
                                    ButtonVariant::Ghost
                                },
                                ButtonSize::Icon,
                                None,
                                Some("layout-grid"),
                                false,
                                Some(30.0),
                                None,
                                move |_, _, cx| {
                                    store_v.update(cx, |s, _| s.explore_list = false);
                                    cx.notify(store_v.entity_id());
                                },
                            )
                        })
                        .child({
                            let store_v = store_view.clone();
                            button_inner(
                                &theme,
                                if list {
                                    ButtonVariant::Secondary
                                } else {
                                    ButtonVariant::Ghost
                                },
                                ButtonSize::Icon,
                                None,
                                Some("list"),
                                false,
                                Some(30.0),
                                None,
                                move |_, _, cx| {
                                    store_v.update(cx, |s, _| s.explore_list = true);
                                    cx.notify(store_v.entity_id());
                                },
                            )
                        }),
                )
                ,
        )
        .child(cat_row)
        .child(
            div()
                .mt(px(20.0))
                .mb(px(20.0))
                .text_size(px(11.0))
                .text_color(theme.muted_fg)
                .child(format!("{} applications", apps.len())),
        )
        .child(if apps.is_empty() {
            empty_state(
                &theme,
                "No applications found",
                "Try a different search or category.",
            )
        } else {
            let _ = store_clear;
            empty_app_grid(&theme, &store, &apps, list, cx)
        })
}

pub fn installed_page(theme: Theme, store: StoreHandle, cx: &mut App) -> Div {
    let (search, filter, installed_count) = {
        let s = store.read(cx);
        (
            s.installed_search.clone(),
            s.installed_filter.clone(),
            s.apps.iter().filter(|a| a.installed.is_some()).count(),
        )
    };
    let focused = store.read(cx).active_input == ActiveInput::InstalledSearch;

    let apps: Vec<StoreApp> = store
        .read(cx)
        .apps
        .iter()
        .filter(|a| {
            let text = format!("{} {}", a.name, a.description).to_lowercase();
            let match_search = text.contains(&search.to_lowercase());
            let match_filter =
                filter != "Updates available" || a.installed.as_deref() != Some(a.version.as_str());
            a.installed.is_some() && match_search && match_filter
        })
        .cloned()
        .collect();

    let store_focus = store.clone();
    let store_filter = store.clone();

    let mut list = div().flex().flex_col();
    for app in &apps {
        let store_open = store.clone();
        let id = app.id.clone();
        let store_a = store.clone();
        let store_menu = store.clone();
        let app_c = app.clone();
        let has_update = app.installed.as_deref() != Some(app.version.as_str());

        list = list.child(
            div()
                .flex()
                .items_center()
                .gap(px(16.0))
                .border_b(px(1.0))
                .border_color(theme.border)
                .py(px(20.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(13.0))
                        .min_w(px(220.0))
                        .cursor_pointer()
                        .child(app_icon(&theme, app, false))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(5.0))
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.foreground)
                                        .child(app.name.clone()),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(5.0))
                                        .text_size(px(10.0))
                                        .text_color(theme.muted_fg)
                                        .child(app.category.label().to_string())
                                        .child("·".to_string())
                                        .child(format!(
                                            "v{}",
                                            app.installed.clone().unwrap_or_default()
                                        )),
                                ),
                        )
                        .id(SharedString::from(format!("exrow-{}", id))).on_click(move |_, _, cx| {
                            store_open.update(cx, |s, _| {
                                s.navigate(Route::Details, Some(id.clone()));
                            });
                            cx.notify(store_open.entity_id());
                        }),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.0))
                        .child(
                            div()
                                .text_size(px(9.0))
                                .text_color(theme.muted_fg)
                                .child(format!(
                                    r"C:\Program Files\XD\{}",
                                    app.name
                                )),
                        )
                        .child(
                            div()
                                .mt(px(6.0))
                                .text_size(px(8.0))
                                .text_color(theme.muted_fg)
                                .child("Installed November 1, 2026".to_string()),
                        ),
                )
                .child(
                    div()
                        .text_size(px(9.0))
                        .text_color(if has_update {
                            theme.primary
                        } else {
                            theme.success
                        })
                        .child(if has_update {
                            "Update available".to_string()
                        } else {
                            "Up to date".to_string()
                        }),
                )
                .child(app_action(&theme, &store_a, app, cx))
                .child({
                    let store_m = store_menu.clone();
                    let app_m = app_c.clone();
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(36.0))
                        .rounded_md()
                        .text_color(theme.muted_fg)
                        .hover(|s| s.bg(theme.accent))
                        .child(icon("more-horizontal").size(px(16.0)).text_color(theme.muted_fg))
                        .id(SharedString::from(format!("exmore-{}", app_m.id))).on_click(move |_, _, cx| {
                            store_m.update(cx, |s, _| {
                                if app_m.installed.is_some() {
                                    s.confirmation = Some(crate::state::Confirmation {
                                        action: crate::state::ConfirmationAction::Uninstall,
                                        app: Some(app_m.clone()),
                                    });
                                    s.dialog = crate::state::DialogKind::Confirmation;
                                } else {
                                    s.toast(
                                        format!("Repository URL for {}", app_m.name),
                                        Some(crate::data::repository(&app_m)),
                                        crate::state::ToastKind::Info,
                                    );
                                }
                            });
                            cx.notify(store_m.entity_id());
                        })
                }),
        );
    }

    div()
        .flex()
        .flex_col()
        .child(page_intro(
            &theme,
            "Your desktop, equipped.",
            &format!(
                "{installed_count} installed applications. All your essentials, ready to go."
            ),
            None,
        ))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(14.0))
                .mb(px(10.0))
                .child(text_field(
                    &theme,
                    &search,
                    "Search applications...",
                    None,
                    focused,
                    move |cx| {
                        store_focus.update(cx, |s, _| {
                            s.focus_input(ActiveInput::InstalledSearch);
                        });
                        cx.notify(store_focus.entity_id());
                    },
                ))
                .child(select(
                    &theme,
                    &filter,
                    &["All installed", "Updates available"],
                    Some(200.0),
                    move |v, cx| {
                        store_filter.update(cx, |s, _| {
                            s.installed_filter = v;
                        });
                        cx.notify(store_filter.entity_id());
                    },
                )),
        )
        .child(if apps.is_empty() {
            empty_state(
                &theme,
                "No installed applications",
                "Explore the collection to add your first application.",
            )
        } else {
            list
        })
}

