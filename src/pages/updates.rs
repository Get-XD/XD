use crate::components::*;
use crate::data::*;
use crate::icons::icon;
use crate::state::{Confirmation, ConfirmationAction, DialogKind, Route, StoreHandle, ToastKind};
use crate::theme::Theme;
use crate::ui::*;
use gpui::{App, Div, FontWeight, InteractiveElement, ParentElement, SharedString, StatefulInteractiveElement, Styled, div, px};

pub fn updates_page(theme: Theme, store: StoreHandle, cx: &mut App) -> Div {
    let updates = store.read(cx).updates();
    let count = updates.len();

    let store_all = store.clone();
    let action_div = if count > 0 {
        Some(button(
            &theme,
            ButtonVariant::Default,
            ButtonSize::Default,
            Some("Update all"),
            Some("refresh-cw"),
            false,
            move |_, _, cx| {
                store_all.update(cx, |s, _| {
                    s.confirmation = Some(Confirmation {
                        action: ConfirmationAction::UpdateAll,
                        app: None,
                    });
                    s.dialog = DialogKind::Confirmation;
                });
                cx.notify(store_all.entity_id());
            },
        ))
    } else {
        None
    };

    let title = if count > 0 {
        "Good tools keep getting better."
    } else {
        "You're all up to date."
    };

    let mut cards = div().flex().flex_wrap().gap(px(18.0));

    if count == 0 {
        cards = cards.child(empty_state(
            &theme,
            "You're all up to date.",
            "Your software is ready for whatever comes next.",
        ));
    } else {
        for app in &updates {
            let store_view = store.clone(); let store_view2 = store_view.clone();
            let store_up = store.clone();
            let store_ig = store.clone();
            let id = app.id.clone();

            cards = cards.child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(460.0))
                    .max_w_full()
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.card)
                    .rounded(px(10.0))
                    .p(px(24.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(13.0))
                            .child(app_icon(&theme, app, false))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(7.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.foreground)
                                            .cursor_pointer()
                                            .child(app.name.clone())
                                            .id(SharedString::from(format!("updrow-{}", app.id))).on_click(move |_, _, cx| {
                                                store_view.update(cx, |s, _| {
                                                    s.navigate(
                                                        Route::Details,
                                                        Some(id.clone()),
                                                    );
                                                });
                                                cx.notify(store_view.entity_id());
                                            }),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(7.0))
                                            .text_size(px(10.0))
                                            .text_color(theme.muted_fg)
                                            .child(format!(
                                                "v{}",
                                                app.installed.clone().unwrap_or_default()
                                            ))
                                            .child(icon("arrow-right").size(px(13.0)).text_color(theme.muted_fg))
                                            .child(
                                                div()
                                                    .text_color(theme.primary)
                                                    .child(format!("v{}", app.version)),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .ml_auto()
                                    .child(app_action(&theme, &store_up, app, cx)),
                            ),
                    )
                    .child(
                        div()
                            .py(px(19.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .text_size(px(8.0))
                                    
                                    .text_color(theme.foreground)
                                    .child("WHAT'S NEW".to_string())
                                    .child(
                                        div()
                                            .text_color(theme.muted_fg)
                                            
                                            .child(format!("{} · {} MB", app.updated, app.size)),
                                    ),
                            )
                            .child(
                                div()
                                    .mt(px(10.0))
                                    .text_size(px(11.0))
                                    .line_height(px(19.8))
                                    .text_color(theme.muted_fg)
                                    .child(
                                        "Performance improvements, a more refined interface, and fixes that make your daily workflow smoother."
                                            .to_string(),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .pt(px(12.0))
                            .border_t(px(1.0))
                            .border_color(theme.border)
                            .child({
                                let s = store_view2.clone();
                                let id2 = app.id.clone();
                                button_inner(
                                    &theme,
                                    ButtonVariant::Ghost,
                                    ButtonSize::Sm,
                                    Some("View release notes"),
                                    Some("arrow-up-right"),
                                    false,
                                    Some(28.0),
                                    Some(10.0),
                                    move |_, _, cx| {
                                        s.update(cx, |st, _| {
                                            st.navigate(Route::Details, Some(id2.clone()));
                                        });
                                        cx.notify(s.entity_id());
                                    },
                                )
                            })
                            .child({
                                let s = store_ig.clone();
                                let id2 = app.id.clone();
                                let name = app.name.clone();
                                button_inner(
                                    &theme,
                                    ButtonVariant::Ghost,
                                    ButtonSize::Sm,
                                    Some("Ignore this update"),
                                    None,
                                    false,
                                    Some(28.0),
                                    Some(10.0),
                                    move |_, _, cx| {
                                        s.update(cx, |st, _| {
                                            st.dispatch(StoreAction::Ignore(id2.clone()));
                                            st.toast(
                                                format!(
                                                    "{name} update ignored for this session"
                                                ),
                                                None,
                                                ToastKind::Info,
                                            );
                                        });
                                        cx.notify(s.entity_id());
                                    },
                                )
                            }),
                    ),
            );
        }
    }

    div()
        .flex()
        .flex_col()
        .child(page_intro(
            &theme,
            title,
            &format!("{count} updates available for your installed applications."),
            action_div,
        ))
        .child(cards)
}
