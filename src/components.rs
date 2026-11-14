use crate::data::*;
use crate::icons::icon;
use crate::state::{Confirmation, ConfirmationAction, Route, StoreHandle};
use crate::theme::Theme;
use crate::ui::*;
use gpui::{App, Div, FontWeight, InteractiveElement, ParentElement, SharedString, Stateful, StatefulInteractiveElement, Styled, div, px};
use gpui::prelude::{FluentBuilder};

pub fn app_icon(theme: &Theme, app: &StoreApp, large: bool) -> Div {
    let size = if large { 90.0 } else { 44.0 };
    let radius = if large { 21.0 } else { 11.0 };
    let icon_sz = if large { 50.0 } else { 25.0 };
    let base = theme.icon_color(&app.color);
    // approximate gradient: lighter top-left, darker bottom-right
    let top = crate::theme::lighten(base, 0.18);
    let bottom = crate::theme::darken(base, 0.12);
    div()
        .flex()
        .items_center()
        .justify_center()
        .flex_shrink_0()
        .w(px(size))
        .h(px(size))
        .rounded(px(radius))
        .bg(bottom)
        .shadow_md()
        .child(
            div()
                .absolute()
                .inset_0()
                .rounded(px(radius))
                .bg(top.opacity(0.55)),
        )
        .child(
            icon(&app.icon)
                .size(px(icon_sz))
                .text_color(theme.primary_fg)
                .relative(),
        )
}

pub fn app_icon_sized(theme: &Theme, app: &StoreApp, size: f32) -> Div {
    let radius = size * 0.25;
    let icon_sz = size * 0.57;
    let base = theme.icon_color(&app.color);
    // approximate gradient: lighter top-left, darker bottom-right
    let top = crate::theme::lighten(base, 0.18);
    let bottom = crate::theme::darken(base, 0.12);
    div()
        .flex()
        .items_center()
        .justify_center()
        .flex_shrink_0()
        .w(px(size))
        .h(px(size))
        .rounded(px(radius))
        .bg(bottom)
        .shadow_md()
        .child(
            div()
                .absolute()
                .inset_0()
                .rounded(px(radius))
                .bg(top.opacity(0.55)),
        )
        .child(
            icon(&app.icon)
                .size(px(icon_sz))
                .text_color(theme.primary_fg)
                .relative(),
        )
}

pub fn app_action(
    theme: &Theme,
    store: &StoreHandle,
    app: &StoreApp,
    _cx: &mut App,
) -> Stateful<Div> {
    let updates_available = app
        .installed
        .as_ref()
        .map(|i| i != &app.version)
        .unwrap_or(false);
    let installed = app.installed.is_some();

    let (variant, label, icon_name) = if updates_available {
        (ButtonVariant::Default, "Update", "refresh-cw")
    } else if installed {
        (ButtonVariant::Secondary, "Open", "arrow-up-right")
    } else {
        (ButtonVariant::Outline, "Install", "download")
    };

    let app_clone = app.clone();
    let store_clone = store.clone();
    let icon_fg = if matches!(variant, ButtonVariant::Default) {
        theme.primary_fg
    } else {
        theme.foreground
    };
    button_inner(
        theme,
        variant,
        ButtonSize::Sm,
        Some(label),
        None,
        app.unavailable,
        Some(27.0),
        Some(9.0),
        move |_, _, cx| {
            if updates_available {
                store_clone.update(cx, |s, _| {
                    s.confirmation = Some(Confirmation {
                        action: ConfirmationAction::Update,
                        app: Some(app_clone.clone()),
                    });
                    s.dialog = crate::state::DialogKind::Confirmation;
                });
            } else if installed {
                store_clone.update(cx, |s, _| {
                    s.toast(
                        format!("{} launch preview", app_clone.name),
                        Some("Desktop launching is not available in this prototype.".into()),
                        crate::state::ToastKind::Info,
                    );
                });
            } else {
                store_clone.update(cx, |s, _| {
                    s.confirmation = Some(Confirmation {
                        action: ConfirmationAction::Install,
                        app: Some(app_clone.clone()),
                    });
                    s.dialog = crate::state::DialogKind::Confirmation;
                });
            }
            cx.notify(store_clone.entity_id());
        },
    )
    .px(px(9.0))
    .gap(px(5.0))
    .child(icon(icon_name).size(px(11.0)).text_color(icon_fg))
}

pub fn app_card(theme: &Theme, store: &StoreHandle, app: &StoreApp, list: bool, cx: &mut App) -> Div {
    let app_c = app.clone();
    let store_c = store.clone();
    let open = store_c.clone();
    let open_id = app.id.clone();

    let main = div()
        .flex()
        .items_center()
        .gap(px(11.0))
        .min_w(px(0.0))
        .cursor_pointer()
        .child(app_icon(theme, app, false))
        .child(
            div()
                .flex_1()
                .min_w(px(0.0))
                .child(
                    div()
                        .text_size(px(10.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.foreground)
                        .child(app.name.clone()),
                )
                .child(
                    div()
                        .mt(px(3.0))
                        .text_size(px(8.0))
                        .text_color(theme.muted_fg)
                        .child(app.category.label().to_string()),
                ),
        )
        .id(SharedString::from(format!("cardmain-{}", open_id))).on_click(move |_, _, cx| {
            open.update(cx, |s, _| {
                s.navigate(Route::Details, Some(open_id.clone()));
            });
            cx.notify(open.entity_id());
        });

    let mut card = div()
        .flex()
        .flex_col()
        .border_1()
        .border_color(theme.border)
        .rounded(px(10.0))
        .px(px(15.0))
        .pt(px(16.0))
        .pb(px(14.0))
        .bg(theme.card)
        .min_w(px(0.0))
        .hover(|s| s.border_color(theme.muted_fg.opacity(0.6)));

    if list {
        card = card
            .flex_row()
            .items_center()
            .gap(px(20.0))
            .child(main)
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .text_size(px(9.0))
                    .line_height(px(16.2))
                    .text_color(theme.muted_fg)
                    .child(app.description.clone()),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(5.0))
                            .text_size(px(8.0))
                            .text_color(theme.muted_fg)
                            .child(format!("v{}", app.version))
                            .when(app.installed.is_some(), |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(3.0))
                                        .text_color(theme.success)
                                        .child(
                                            icon("check")
                                                .size(px(12.0))
                                                .text_color(theme.success),
                                        )
                                        .child("Installed"),
                                )
                            }),
                    )
                    .child(app_action(theme, store, app, cx)),
            );
    } else {
        card = card
            .child(main)
            .child(
                div()
                    .mt(px(15.0))
                    .text_size(px(9.0))
                    .line_height(px(16.2))
                    .text_color(theme.muted_fg)
                    .h(px(33.0))
                    .overflow_hidden()
                    .child(app.description.clone()),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .mt(px(18.0))
                    .gap(px(5.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(5.0))
                            .text_size(px(8.0))
                            .text_color(theme.muted_fg)
                            .child(format!("v{}", app.version))
                            .when(app.installed.is_some(), |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(3.0))
                                        .text_color(theme.success)
                                        .child(
                                            icon("check")
                                                .size(px(12.0))
                                                .text_color(theme.success),
                                        )
                                        .child("Installed"),
                                )
                            }),
                    )
                    .child(app_action(theme, store, app, cx)),
            );
    }

    // keep app_c used
    let _ = app_c;
    card
}

pub fn empty_app_grid(theme: &Theme, store: &StoreHandle, apps: &[StoreApp], list: bool, cx: &mut App) -> Div {
    if list {
        let mut col = div().flex().flex_col().gap(px(10.0));
        for app in apps {
            col = col.child(app_card(theme, store, app, true, cx));
        }
        col
    } else {
        // grid approximated with wrap
        let mut row = div().flex().flex_wrap().gap(px(13.0));
        for app in apps {
            row = row.child(
                div()
                    .w(px(240.0))
                    .flex_1()
                    .min_w(px(220.0))
                    .child(app_card(theme, store, app, false, cx)),
            );
        }
        row
    }
}
