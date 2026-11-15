use crate::components::*;
use crate::data::*;
use crate::icons::icon;
use crate::state::{Confirmation, ConfirmationAction, DialogKind, Route, StoreHandle, ToastKind};
use crate::theme::Theme;
use crate::ui::*;
use gpui::{App, Div, FontWeight, InteractiveElement, ObjectFit, ParentElement, SharedString, StatefulInteractiveElement, Styled, StyledImage, div, img, px};
use gpui::prelude::{FluentBuilder};

pub fn details_page(theme: Theme, store: StoreHandle, cx: &mut App) -> Div {
    let (app, tab, shot) = {
        let s = store.read(cx);
        let app = s
            .detail_app_id
            .clone()
            .and_then(|id| s.find_app(&id).cloned());
        (app, s.details_tab.clone(), s.details_shot)
    };

    let Some(app) = app else {
        return empty_state(
            &theme,
            "Application not found",
            "This application isn't in the local catalog.",
        );
    };

    let store_back = store.clone();
    let store_tab = store.clone();
    let store_shot = store.clone();
    let store_action = store.clone();
    let store_uninstall = store.clone();
    let store_copy = store.clone(); let store_copy2 = store_copy.clone();
    let store_changelog = store.clone();

    let mut root = div().flex().flex_col();

    // back
    root = root.child(
        button_inner(
            &theme,
            ButtonVariant::Ghost,
            ButtonSize::Sm,
            Some("Back to Explore"),
            Some("chevron-left"),
            false,
            Some(32.0),
            Some(11.0),
            move |_, _, cx| {
                store_back.update(cx, |s, _| s.navigate(Route::Explore, None));
                cx.notify(store_back.entity_id());
            },
        ),
    );

    // heading
    root = root.child(
        div()
            .flex()
            .items_center()
            .gap(px(25.0))
            .my(px(30.0))
            .child(app_icon(&theme, &app, true))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(8.0))
                            
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.muted_fg)
                            .mb(px(6.0))
                            .child(app.category.label().to_uppercase()),
                    )
                    .child(
                        div()
                            .text_size(px(30.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.foreground)
                            .child(app.name.clone()),
                    )
                    .child(
                        div()
                            .mt(px(6.0))
                            .text_size(px(13.0))
                            .text_color(theme.muted_fg)
                            .child(app.tagline.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(5.0))
                            .mt(px(10.0))
                            .text_size(px(11.0))
                            .text_color(theme.muted_fg)
                            .child("By XD".to_string())
                            .child(
                                icon("shield-check")
                                    .size(px(13.0))
                                    .text_color(theme.primary),
                            ),
                    ),
            )
            .child(
                div()
                    .ml_auto()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(app_action(&theme, &store_action, &app, cx))
                    .child(
                        div()
                            .mt(px(9.0))
                            .flex()
                            .gap(px(5.0))
                            .text_size(px(10.0))
                            .text_color(theme.muted_fg)
                            .child("Free".to_string())
                            .child("·".to_string())
                            .child(format!("{} MB", app.size)),
                    ),
            ),
    );

    // tabs
    let mut tabs = div()
        .flex()
        .gap(px(15.0))
        .mb(px(25.0))
        .border_b(px(1.0))
        .border_color(theme.border);
    for t in ["Overview", "Release notes"] {
        let selected = tab == t;
        let s = store_tab.clone();
        let label = t.to_string();
        tabs = tabs.child(
            div()
                .relative()
                .h(px(43.0))
                .px(px(15.0))
                .flex()
                .items_center()
                .text_size(px(12.0))
                .text_color(if selected {
                    theme.foreground
                } else {
                    theme.muted_fg
                })
                .cursor_pointer()
                .when(selected, |d| {
                    d.child(
                        div()
                            .absolute()
                            .bottom(px(0.0))
                            .left(px(15.0))
                            .right(px(15.0))
                            .h(px(2.0))
                            .bg(theme.primary),
                    )
                })
                .child(label.clone())
                .id(SharedString::from(format!("details-tab-{}", label))).on_click(move |_, _, cx| {
                    s.update(cx, |st, _| st.details_tab = label.clone());
                    cx.notify(s.entity_id());
                }),
        );
    }
    root = root.child(tabs);

    // layout
    let mut main_col = div().flex_1().min_w(px(0.0)).flex().flex_col();

    if tab == "Overview" {
        main_col = main_col
            .child(
                div()
                    .text_size(px(17.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.foreground)
                    .child("About this application".to_string()),
            )
            .child(
                div()
                    .mt(px(16.0))
                    .text_size(px(12.0))
                    .line_height(px(22.8))
                    .text_color(theme.muted_fg)
                    .child(app.description.clone()),
            )
            .child(
                div()
                    .mt(px(32.0))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .text_size(px(17.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.foreground)
                    .child("Screenshots".to_string())
                    .child(
                        div()
                            .text_size(px(8.0))
                            .font_weight(FontWeight::NORMAL)
                            .text_color(theme.muted_fg)
                            .child("CONCEPT PREVIEWS".to_string()),
                    ),
            )
            .child(
                div()
                    .mt(px(18.0))
                    .border_1()
                    .border_color(theme.border)
                    .rounded(px(10.0))
                    .overflow_hidden()
                    .h(px(280.0))
                    .when(app.id == "soundcore", |d| {
                        d.child(
                            img(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/soundcore-feature.jpg"))
                                .w_full()
                                .h_full()
                                .object_fit(ObjectFit::Cover),
                        )
                    })
                    .when(app.id != "soundcore", |d| {
                        d.bg(theme.card).p(px(15.0)).child(concept_preview(
                            &theme,
                            &app,
                            shot,
                        ))
                    }),
            )
            .child(
                div()
                    .flex()
                    .gap(px(6.0))
                    .mt(px(10.0))
                    .child({
                        let s = store_shot.clone();
                        button_inner(
                            &theme,
                            if shot == 0 {
                                ButtonVariant::Secondary
                            } else {
                                ButtonVariant::Ghost
                            },
                            ButtonSize::Sm,
                            Some("Workspace"),
                            None,
                            false,
                            Some(32.0),
                            Some(10.0),
                            move |_, _, cx| {
                                s.update(cx, |st, _| st.details_shot = 0);
                                cx.notify(s.entity_id());
                            },
                        )
                    })
                    .child({
                        let s = store_shot.clone();
                        let app_id = app.id.clone();
                        button_inner(
                            &theme,
                            if shot == 1 {
                                ButtonVariant::Secondary
                            } else {
                                ButtonVariant::Ghost
                            },
                            ButtonSize::Sm,
                            Some("Activity"),
                            None,
                            false,
                            Some(32.0),
                            Some(10.0),
                            move |_, _, cx| {
                                s.update(cx, |st, _| {
                                    st.details_shot = 1;
                                    if app_id == "soundcore" {
                                        st.details_tab = "Release notes".into();
                                    }
                                });
                                cx.notify(s.entity_id());
                            },
                        )
                    }),
            );
    } else {
        main_col = main_col
            .child(
                div()
                    .text_size(px(17.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.foreground)
                    .child(format!("What's new in v{}", app.version)),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(12.0))
                    .text_color(theme.muted_fg)
                    .child(format!("Released {}", app.updated)),
            )
            .child({
                let mut ul = div()
                    .mt(px(24.0))
                    .pl(px(20.0))
                    .text_size(px(12.0))
                    .line_height(px(33.6))
                    .text_color(theme.muted_fg);
                for item in [
                    "Refined application interface and navigation.",
                    "Faster startup and reduced memory usage.",
                    "Improved reliability and fixed reported issues.",
                    "Better compatibility with Windows 11.",
                ] {
                    ul = ul.child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .child(div().size(px(4.0)).rounded_full().bg(theme.muted_fg))
                            .child(item.to_string()),
                    );
                }
                ul
            })
            .child(
                div()
                    .mt(px(20.0))
                    .text_size(px(8.0))
                    .line_height(px(14.4))
                    .text_color(theme.muted_fg)
                    .child("SAMPLE RELEASE NOTES FOR THIS PROTOTYPE".to_string()),
            );
    }

    // aside
    let mut facts = div().flex().flex_col().gap(px(19.0));
    for (label, value) in [
                        ("Developer", "XD".to_string()),
        ("Latest version", app.version.clone()),
        (
            "Installed version",
            app.installed.clone().unwrap_or_else(|| "Not installed".into()),
        ),
        (
            "Compatibility",
            "Windows 10 / 11 · x64".to_string(),
        ),
        ("Download size", format!("{} MB", app.size)),
        ("Last updated", app.updated.clone()),
        ("License", "MIT (demo)".to_string()),
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

    let repo = repository(&app);
    let repo2 = repo.clone();
    let app_un = app.clone();

    let aside = div()
        .w(px(270.0))
        .flex_shrink_0()
        .pl(px(25.0))
        .border_l(px(1.0))
        .border_color(theme.border)
        .flex()
        .flex_col()
        .child(
            div()
                .text_size(px(13.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.foreground)
                .mb(px(20.0))
                .child("Application information".to_string()),
        )
        .child(facts)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(7.0))
                .mt(px(24.0))
                .child(button(
                    &theme,
                    ButtonVariant::Outline,
                    ButtonSize::Default,
                    Some("GitHub repository"),
                    Some("github"),
                    false,
                    {
                        let r = repo.clone();
                        move |_, _, cx| {
                            store_copy.update(cx, |s, _| {
                                s.toast(
                                    "Repository URL",
                                    Some(r.clone()),
                                    ToastKind::Info,
                                );
                            });
                            cx.notify(store_copy.entity_id());
                        }
                    },
                ))
                .child(button(
                    &theme,
                    ButtonVariant::Ghost,
                    ButtonSize::Default,
                    Some("Copy repository URL"),
                    Some("copy"),
                    false,
                    {
                        let r = repo2.clone();
                        move |_, _, cx| {
                            store_copy2.update(cx, |s, _| {
                                s.toast("Repository URL copied", Some(r.clone()), ToastKind::Success);
                            });
                            cx.notify(store_copy2.entity_id());
                        }
                    },
                ))
                .child(button(
                    &theme,
                    ButtonVariant::Ghost,
                    ButtonSize::Default,
                    Some("View changelog"),
                    Some("list"),
                    false,
                    move |_, _, cx| {
                        store_changelog.update(cx, |s, _| {
                            s.details_tab = "Release notes".into();
                        });
                        cx.notify(store_changelog.entity_id());
                    },
                ))
                .when(app.installed.is_some(), |d| {
                    d.child(button(
                        &theme,
                        ButtonVariant::Ghost,
                        ButtonSize::Default,
                        Some("Uninstall application"),
                        Some("trash-2"),
                        false,
                        move |_, _, cx| {
                            store_uninstall.update(cx, |s, _| {
                                s.confirmation = Some(Confirmation {
                                    action: ConfirmationAction::Uninstall,
                                    app: Some(app_un.clone()),
                                });
                                s.dialog = DialogKind::Confirmation;
                            });
                            cx.notify(store_uninstall.entity_id());
                        },
                    ))
                }),
        )
        .child(
            div()
                .mt(px(20.0))
                .text_size(px(8.0))
                .line_height(px(14.4))
                .text_color(theme.muted_fg)
                .child(
                    "Repository addresses and release information are sample data.".to_string(),
                ),
        );

    root = root.child(
        div()
            .flex()
            .flex_row()
            .gap(px(35.0))
            .child(main_col)
            .child(aside),
    );

    root
}

fn concept_preview(theme: &Theme, app: &StoreApp, shot: usize) -> Div {
    let base = theme.icon_color(&app.color);
    div()
        .h_full()
        .flex()
        .flex_col()
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(5.0))
                .pb(px(12.0))
                .border_b(px(1.0))
                .border_color(theme.border)
                .child(div().size(px(5.0)).rounded_full().bg(theme.muted_fg))
                .child(div().size(px(5.0)).rounded_full().bg(theme.muted_fg))
                .child(div().size(px(5.0)).rounded_full().bg(theme.muted_fg))
                .child(
                    div()
                        .ml(px(20.0))
                        .text_size(px(9.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.foreground)
                        .child(app.name.clone()),
                ),
        )
        .child(
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .gap(px(30.0))
                .child(app_icon(theme, app, true))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(16.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.foreground)
                                .child(if shot == 0 {
                                    app.tagline.clone()
                                } else {
                                    "Everything, at a glance.".into()
                                }),
                        )
                        .child(
                            div()
                                .mt(px(10.0))
                                .text_size(px(10.0))
                                .text_color(theme.muted_fg)
                                .child(if shot == 0 {
                                    "Your personal workspace".to_string()
                                } else {
                                    "A cleaner way to manage your day".to_string()
                                }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(10.0))
                                .mt(px(20.0))
                                .child(
                                    div()
                                        .h(px(8.0))
                                        .w(px(150.0))
                                        .rounded(px(4.0))
                                        .bg(theme.secondary),
                                )
                                .child(
                                    div()
                                        .h(px(8.0))
                                        .w(px(110.0))
                                        .rounded(px(4.0))
                                        .bg(theme.secondary),
                                )
                                .child(
                                    div()
                                        .h(px(8.0))
                                        .w(px(150.0))
                                        .rounded(px(4.0))
                                        .bg(theme.secondary),
                                ),
                        ),
                ),
        )
        .when(false, |d| d.bg(base))
}
