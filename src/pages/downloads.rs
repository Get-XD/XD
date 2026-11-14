use crate::components::*;
use crate::data::*;
use crate::icons::icon;
use crate::state::{Confirmation, ConfirmationAction, DialogKind, Route, StoreHandle, ToastKind};
use crate::theme::Theme;
use crate::ui::*;
use gpui::{App, Div, FontWeight, InteractiveElement, ParentElement, SharedString, StatefulInteractiveElement, Styled, Transformation, div, px, radians};
use gpui::prelude::{FluentBuilder};

pub fn downloads_page(theme: Theme, store: StoreHandle, cx: &mut App) -> Div {
    let downloads = store.read(cx).downloads.clone();
    let apps = store.read(cx).apps.clone();

    let store_clear = store.clone();

    let mut list = div().flex().flex_col();
    for d in &downloads {
        let Some(app) = apps.iter().find(|a| a.id == d.id) else {
            continue;
        };

        let status_color = match d.status {
            DownloadStatus::Failed => theme.destructive,
            DownloadStatus::Completed => theme.success,
            _ => theme.muted_fg,
        };

        let store_pause = store.clone();
        let store_resume = store.clone();
        let store_cancel = store.clone();
        let store_status = store.clone();
        let store_complete = store.clone();
        let store_open = store.clone();
        let id = d.id.clone();
        let id2 = d.id.clone();
        let id3 = d.id.clone();
        let id4 = d.id.clone();
        let app_c = app.clone();

        let mut row = div()
            .flex()
            .gap(px(20.0))
            .items_start()
            .border_b(px(1.0))
            .border_color(theme.border)
            .py(px(25.0))
            .child(app_icon(&theme, app, false))
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.foreground)
                                    .cursor_pointer()
                                    .child(app.name.clone())
                                    .id(SharedString::from(format!("dl-{}", id))).on_click(move |_, _, cx| {
                                        store_open.update(cx, |s, _| {
                                            s.navigate(Route::Details, Some(id.clone()));
                                        });
                                        cx.notify(store_open.entity_id());
                                    }),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(status_color)
                                    .child(d.status.label().to_string()),
                            ),
                    )
                    .child(
                        div()
                            .mt(px(18.0))
                            .mb(px(10.0))
                            .w_full()
                            .h(px(5.0))
                            .rounded(px(4.0))
                            .bg(theme.secondary)
                            .child(
                                div()
                                    .h_full()
                                    .w(px((d.progress as f32 / 100.0) * 600.0))
                                    .max_w_full()
                                    .rounded(px(4.0))
                                    .bg(theme.primary),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(12.0))
                            .text_size(px(10.0))
                            .text_color(theme.muted_fg)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(7.0))
                                    .child(format!(
                                        "{} MB / {} MB",
                                        (app.size as u32 * d.progress) / 100,
                                        app.size
                                    ))
                                    .child("·".to_string())
                                    .child(format!("{}%", d.progress)),
                            )
                            .child(
                                div().child(if d.status == DownloadStatus::Downloading {
                                    "4.2 MB/s (demo)".to_string()
                                } else {
                                    "No active transfer".to_string()
                                }),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .mt(px(14.0))
                            .text_size(px(9.0))
                            .text_color(theme.muted_fg)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child("Demo status".to_string())
                                    .child({
                                        let cur = d.status.clone();
                                        let opts: Vec<String> =
                                            DownloadStatus::all().iter().map(|s| s.label().to_string()).collect();
                                        let cur_s = cur.label().to_string();
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(6.0))
                                            .px(px(7.0))
                                            .py(px(4.0))
                                            .border_1()
                                            .border_color(theme.border)
                                            .rounded(px(6.0))
                                            .bg(theme.card)
                                            .text_size(px(9.0))
                                            .text_color(theme.foreground)
                                            .cursor_pointer()
                                            .child(cur_s.clone())
                                            .child(
                                                icon("chevron-right")
                                                    .size(px(10.0))
                                                    .text_color(theme.muted_fg)
                                                    .with_transformation(
                                                        Transformation::rotate(radians(std::f32::consts::FRAC_PI_2)),
                                                    ),
                                            )
                                            .id(SharedString::from(format!("dlstatus-{}", d.id))).on_click(move |_, _, cx| {
                                                if let Some(ix) = opts
                                                    .iter()
                                                    .position(|o| *o == cur_s)
                                                {
                                                    let next_label =
                                                        opts[(ix + 1) % opts.len()].clone();
                                                    if let Some(st) = DownloadStatus::all()
                                                        .iter()
                                                        .find(|s| s.label() == next_label)
                                                    {
                                                        let st = st.clone();
                                                        let id = id2.clone();
                                                        store_status.update(cx, |s, _| {
                                                            s.dispatch(StoreAction::Download(
                                                                id, st,
                                                            ));
                                                        });
                                                        cx.notify(store_status.entity_id());
                                                        }
                                                }
                                            })
                                    }),
                            )
                            .when(d.status == DownloadStatus::Installing, |d| {
                                d.child({
                                    let s = store_complete.clone();
                                    let app = app_c.clone();
                                    button(
                                        &theme,
                                        ButtonVariant::Outline,
                                        ButtonSize::Sm,
                                        Some("Complete simulation"),
                                        None,
                                        false,
                                        move |_, _, cx| {
                                            s.update(cx, |st, _| {
                                                st.confirmation = Some(Confirmation {
                                                    action: if app.installed.is_some() {
                                                        ConfirmationAction::Update
                                                    } else {
                                                        ConfirmationAction::Install
                                                    },
                                                    app: Some(app.clone()),
                                                });
                                                st.dialog = DialogKind::Confirmation;
                                            });
                                            cx.notify(s.entity_id());
                                        },
                                    )
                                })
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap(px(3.0))
                    .child({
                        let s = store_pause.clone();
                        let id = id3.clone();
                        button_inner(
                            &theme,
                            ButtonVariant::Ghost,
                            ButtonSize::Icon,
                            None,
                            Some("pause"),
                            !d.status.is_active()
                                || matches!(
                                    d.status,
                                    DownloadStatus::Paused | DownloadStatus::Failed
                                ),
                            Some(32.0),
                            None,
                            move |_, _, cx| {
                                s.update(cx, |st, _| {
                                    st.dispatch(StoreAction::Download(
                                        id.clone(),
                                        DownloadStatus::Paused,
                                    ));
                                });
                                cx.notify(s.entity_id());
                            },
                        )
                    })
                    .child({
                        let s = store_resume.clone();
                        let id = id3.clone();
                        let paused = d.status == DownloadStatus::Paused;
                        button_inner(
                            &theme,
                            ButtonVariant::Ghost,
                            ButtonSize::Icon,
                            None,
                            if paused { Some("play") } else { Some("refresh-cw") },
                            !matches!(
                                d.status,
                                DownloadStatus::Paused
                                    | DownloadStatus::Failed
                                    | DownloadStatus::Cancelled
                            ),
                            Some(32.0),
                            None,
                            move |_, _, cx| {
                                s.update(cx, |st, _| {
                                    st.dispatch(StoreAction::Download(
                                        id.clone(),
                                        DownloadStatus::Downloading,
                                    ));
                                });
                                cx.notify(s.entity_id());
                            },
                        )
                    })
                    .child({
                        let s = store_cancel.clone();
                        let id = id4.clone();
                        button_inner(
                            &theme,
                            ButtonVariant::Ghost,
                            ButtonSize::Icon,
                            None,
                            Some("x"),
                            matches!(
                                d.status,
                                DownloadStatus::Completed | DownloadStatus::Cancelled
                            ),
                            Some(32.0),
                            None,
                            move |_, _, cx| {
                                s.update(cx, |st, _| {
                                    st.dispatch(StoreAction::Download(
                                        id.clone(),
                                        DownloadStatus::Cancelled,
                                    ));
                                });
                                cx.notify(s.entity_id());
                            },
                        )
                    }),
            );

        let _ = &mut row;
        list = list.child(row);
    }

    div()
        .flex()
        .flex_col()
        .child(page_intro(
            &theme,
            "Every download, in view.",
            "Static demonstration entries. No network transfers are running.",
            Some(button(
                &theme,
                ButtonVariant::Outline,
                ButtonSize::Default,
                Some("Clear completed"),
                Some("trash-2"),
                false,
                move |_, _, cx| {
                    store_clear.update(cx, |s, _| {
                        s.dispatch(StoreAction::ClearCompleted);
                        s.toast("Cleared completed downloads", None, ToastKind::Info);
                    });
                    cx.notify(store_clear.entity_id());
                },
            )),
        ))
        .child(if downloads.is_empty() {
            empty_state(
                &theme,
                "A clear queue",
                "Your download activity will appear here.",
            )
        } else {
            list
        })
}
