use crate::components::*;
use crate::data::*;
use crate::icons::icon;
use crate::state::{Route, StoreHandle};
use crate::theme::Theme;
use crate::ui::*;
use gpui::{App, ClickEvent, Div, FontWeight, Hsla, InteractiveElement, ObjectFit, ParentElement, SharedString, Stateful, StatefulInteractiveElement, Styled, StyledImage, Window, div, img, px};
use gpui::prelude::FluentBuilder;

pub fn home_page(theme: Theme, store: StoreHandle, cx: &mut App) -> Div {
    let (updates_count, installed_count, featured, recent, popular, installed_apps) = {
        let s = store.read(cx);
        (
            s.updates().len(),
            s.apps.iter().filter(|a| a.installed.is_some()).count(),
            s.apps.first().cloned(),
            s.apps.iter().take(4).cloned().collect::<Vec<_>>(),
            {
                let mut apps = s.apps.clone();
                apps.sort_by(|a, b| b.popular.cmp(&a.popular));
                apps.into_iter().skip(4).take(4).collect::<Vec<_>>()
            },
            s.apps
                .iter()
                .filter(|a| a.installed.is_some())
                .take(4)
                .cloned()
                .collect::<Vec<_>>(),
        )
    };

    let Some(featured) = featured else {
        return div();
    };

    let mut root = div().flex().flex_col();

    // welcome row
    root = root.child(
        div()
            .flex()
            .items_center()
            .justify_between()
            .mb(px(25.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(8.0))
                            
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.muted_fg)
                            .mb(px(9.0))
                            .child("A WORKSPACE THAT WORKS FOR YOU".to_string()),
                    )
                    .child(
                        div()
                            .text_size(px(25.0))
                            .font_weight(FontWeight::BOLD)
                            .line_height(px(35.0))
                            .text_color(theme.foreground)
                            .child("Your software, all in one place.".to_string()),
                    )
                    .child(
                        div()
                            .mt(px(4.0))
                            .text_size(px(11.0))
                            .text_color(theme.muted_fg)
                            .child(
                                "Thoughtfully built tools. Seamlessly in your workflow.".to_string(),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .px(px(10.0))
                    .py(px(6.0))
                    .border_1()
                    .border_color(theme.border)
                    .rounded(px(20.0))
                    .text_size(px(9.0))
                    .text_color(theme.muted_fg)
                    .child(icon("shield-check").size(px(14.0)).text_color(theme.muted_fg))
                    .child("Built by Pamod".to_string()),
            ),
    );

    // feature hero
    let store_hero = store.clone();
    let hero = featured.clone();
    root = root.child(
        div()
            .id("feature-hero")
            .relative()
            .h(px(300.0))
            .rounded(px(12.0))
            .overflow_hidden()
            .bg(theme.hero_shade)
            .text_color(theme.hero_fg)
            .child(
                img(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/soundcore-feature.jpg"))
                    .absolute()
                    .left(px(0.0))
                    .top(px(0.0))
                    .w_full()
                    .h(px(324.0))
                    .object_fit(ObjectFit::Cover),
            )
            .child(
                div()
                    .relative()
                    
                    .p(px(32.0))
                    .max_w(px(592.0))
                    .flex()
                    .flex_col()
                    .items_start()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(8.0))
                            .py(px(5.0))
                            .gap(px(6.0))
                            .border_1()
                            .border_color(theme.primary.opacity(0.3))
                            .bg(theme.primary.opacity(0.12))
                            .rounded(px(4.0))
                            .text_size(px(8.0))
                            
                            .text_color(theme.hero_muted)
                            .child(icon("sparkles").size(px(13.0)).text_color(theme.hero_muted))
                            .child("FEATURED APPLICATION".to_string()),
                    )
                    .child(
                        div()
                            .mt(px(18.0))
                            .text_size(px(28.0))
                            .font_weight(FontWeight(750.0))
                            .text_color(theme.hero_fg)
                            .child(hero.name.clone()),
                    )
                    .child(
                        div()
                            .mt(px(2.0))
                            .text_size(px(19.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.hero_fg)
                            .child(hero.tagline.clone()),
                    )
                    .child(
                        div()
                            .mt(px(11.0))
                            .text_size(px(10.0))
                            .line_height(px(18.0))
                            .text_color(theme.hero_muted)
                            .child(
                                "Fine-tune your audio, personalize your devices,\n and make every moment sound better.".to_string(),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(16.0))
                            .mt(px(23.0))
                            .child(
                                button_inner(
                                    &theme,
                                    ButtonVariant::Default,
                                    ButtonSize::Default,
                                    Some("Discover Soundcore"),
                                    None,
                                    false,
                                    Some(35.0),
                                    Some(10.0),
                                    move |_, _, cx| {
                                        store_hero.update(cx, |s, _| {
                                            s.navigate(Route::Details, Some(hero.id.clone()));
                                        });
                                        cx.notify(store_hero.entity_id());
                                    },
                                )
                                .px(px(14.0))
                                .child(
                                    icon("arrow-right")
                                        .size(px(16.0))
                                        .text_color(theme.primary_fg),
                                ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(5.0))
                                    .text_size(px(9.0))
                                    .text_color(theme.hero_muted)
                                    .child("v2.4.0".to_string())
                                    .child("·".to_string())
                                    .child("Windows".to_string()),
                            ),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .bottom(px(13.0))
                    .left(px(0.0))
                    .right(px(0.0))
                    .flex()
                    .justify_center()
                    .gap(px(5.0))
                    .child(div().w(px(15.0)).h(px(4.0)).rounded(px(5.0)).bg(theme.primary))
                    .child(
                        div()
                            .w(px(4.0))
                            .h(px(4.0))
                            .rounded(px(5.0))
                            .bg(theme.hero_muted.opacity(0.4)),
                    )
                    .child(
                        div()
                            .w(px(4.0))
                            .h(px(4.0))
                            .rounded(px(5.0))
                            .bg(theme.hero_muted.opacity(0.4)),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .right(px(22.0))
                    .bottom(px(17.0))
                    .text_size(px(6.0))
                    
                    .text_color(theme.hero_muted)
                    .child("DESIGNED FOR YOUR DESKTOP".to_string()),
            ),
    );

    // stat strip
    let store_s1 = store.clone();
    let store_s2 = store.clone();
    let store_s3 = store.clone();
    root = root.child(
        div()
            .grid()
            .grid_cols(3)
            .border_1()
            .border_color(theme.border)
            .rounded(px(8.0))
            .mt(px(17.0))
            .bg(theme.card)
            .child(stat_item(
                &theme,
                "layers",
                &format!("{installed_count} apps installed"),
                "Your everyday essentials",
                theme.secondary,
                theme.muted_fg,
                None,
                false,
                move |_, _, cx| {
                    store_s1.update(cx, |s, _| s.navigate(Route::Installed, None));
                    cx.notify(store_s1.entity_id());
                },
            ))
            .child(stat_item(
                &theme,
                "refresh-cw",
                &format!("{updates_count} updates available"),
                "A little better, with every release",
                theme.blue_soft,
                theme.primary,
                Some("NEW"),
                true,
                move |_, _, cx| {
                    store_s2.update(cx, |s, _| s.navigate(Route::Updates, None));
                    cx.notify(store_s2.entity_id());
                },
            ))
            .child(stat_item(
                &theme,
                "compass",
                "Find your next favorite",
                "Explore the complete collection",
                theme.success.opacity(0.1),
                theme.success,
                None,
                true,
                move |_, _, cx| {
                    store_s3.update(cx, |s, _| s.navigate(Route::Explore, None));
                    cx.notify(store_s3.entity_id());
                },
            )),
    );

    // recently updated
    let store_r = store.clone();
    root = root.child(
        div()
            .mt(px(theme.section_gap()))
            .child(section_heading(
                &theme,
                "Recently updated",
                Some("Fresh improvements to the tools you love."),
                true,
                move |_, _, cx| {
                    store_r.update(cx, |s, _| s.navigate(Route::Explore, None));
                    cx.notify(store_r.entity_id());
                },
            ))
            .child(app_grid(&theme, &store, &recent, cx)),
    );

    // editorial
    let store_dev = store.clone();
    let store_prod = store.clone();
    root = root.child(
        div()
            .mt(px(theme.section_gap()))
            .child(section_heading(
                &theme,
                "Made for your workflow",
                Some("Small tools. A meaningful difference."),
                true,
                {
                    let s = store.clone();
                    move |_, _, cx| {
                        s.update(cx, |st, _| st.navigate(Route::Explore, None));
                        cx.notify(s.entity_id());
                    }
                },
            ))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(14.0))
                    .child(editorial_dev_panel(&theme, move |_, _, cx| {
                        store_dev.update(cx, |s, _| {
                            s.explore_category = "Developer Tools".into();
                            s.navigate(Route::Explore, None);
                        });
                        cx.notify(store_dev.entity_id());
                    }))
                    .child(editorial_focus_panel(&theme, move |_, _, cx| {
                        store_prod.update(cx, |s, _| {
                            s.explore_category = "Productivity".into();
                            s.navigate(Route::Explore, None);
                        });
                        cx.notify(store_prod.entity_id());
                    })),
            ),
    );

    // popular
    let store_p = store.clone();
    root = root.child(
        div()
            .mt(px(theme.section_gap()))
            .child(section_heading(
                &theme,
                "Popular applications",
                Some("Good software, worth a spot on your desktop."),
                true,
                move |_, _, cx| {
                    store_p.update(cx, |s, _| s.navigate(Route::Explore, None));
                    cx.notify(store_p.entity_id());
                },
            ))
            .child(app_grid(&theme, &store, &popular, cx)),
    );

    // installed preview
    let store_i = store.clone();
    root = root.child(
        div()
            .mt(px(theme.section_gap()))
            .child(section_heading(
                &theme,
                "Your installed applications",
                None,
                true,
                move |_, _, cx| {
                    store_i.update(cx, |s, _| s.navigate(Route::Installed, None));
                    cx.notify(store_i.entity_id());
                },
            ))
            .child({
                let mut grid = div().flex().flex_row().flex_wrap().gap(px(0.0)).gap_x(px(25.0));
                for app in &installed_apps {
                    let store_c = store.clone();
                    let id = app.id.clone();
                    let store_a = store.clone();
                    let app_c = app.clone();
                    grid = grid.child(
                        div()
                            .flex_1()
                            .min_w(px(280.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(10.0))
                            .py(px(14.0))
                            .border_b(px(1.0))
                            .border_color(theme.border)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(10.0))
                                    .cursor_pointer()
                                    .child(app_icon_sized(&theme, app, 35.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(4.0))
                                            .child(
                                                div()
                                                    .text_size(px(10.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(theme.foreground)
                                                    .child(app.name.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(8.0))
                                                    .text_color(theme.muted_fg)
                                                    .child(format!(
                                                        "v{}",
                                                        app.installed.clone().unwrap_or_default()
                                                    )),
                                            ),
                                    )
                                    .id(SharedString::from(format!("hcard-{}", id))).on_click(move |_, _, cx| {
                                        store_c.update(cx, |s, _| {
                                            s.navigate(Route::Details, Some(id.clone()));
                                        });
                                        cx.notify(store_c.entity_id());
                                    }),
                            )
                            .child(app_action(&theme, &store_a, &app_c, cx)),
                    );
                }
                grid
            }),
    );

    root
}

fn stat_item(
    theme: &Theme,
    icon_name: &str,
    strong: &str,
    sub: &str,
    symbol_bg: Hsla,
    symbol_fg: Hsla,
    badge: Option<&str>,
    sep: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    div()
        .flex()
        .items_center()
        .gap(px(12.0))
        .px(px(19.0))
        .pt(px(17.0))
        .pb(px(16.0))
        .min_w(px(0.0))
        .cursor_pointer()
        .hover(|s| s.bg(theme.accent.opacity(0.5)))
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .w(px(32.0))
                .h(px(33.0))
                .rounded(px(8.0))
                .bg(symbol_bg)
                .text_color(symbol_fg)
                .child(icon(icon_name).size(px(15.0)).text_color(symbol_fg)),
        )
        .child({
            let mut strong_row = div()
                .flex()
                .items_center()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(10.0))
                        .font_weight(FontWeight(650.0))
                        .text_color(theme.foreground)
                        .child(strong.to_string()),
                );
            if let Some(b) = badge {
                strong_row = strong_row.child(
                    div()
                        .px(px(4.0))
                        .py(px(2.0))
                        .rounded(px(3.0))
                        .bg(theme.blue_soft)
                        .text_size(px(6.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.primary)
                        .child(b.to_string()),
                );
            }
            div()
                .flex_1()
                .min_w(px(0.0))
                .mt(px(3.0))
                .child(strong_row)
                .child(
                    div()
                        .mt(px(4.0))
                        .text_size(px(8.0))
                        .text_color(theme.muted_fg)
                        .child(sub.to_string()),
                )
        })
        .child(
            icon("chevron-right")
                .size(px(13.0))
                .text_color(theme.muted_fg)
                .flex_shrink_0(),
        )
        .id(SharedString::from(format!("stat-{}", icon_name)))
        .on_click(on_click)
        .when(sep, |d| d.border_l(px(1.0)).border_color(theme.border))
}

fn app_grid(theme: &Theme, store: &StoreHandle, apps: &[StoreApp], cx: &mut App) -> Div {
    empty_app_grid(theme, store, apps, false, cx)
}

fn editorial_dev_panel(theme: &Theme, on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Stateful<Div> {
    div()
        .relative()
        .flex()
        .items_center()
        .min_h(px(190.0))
        .flex_1()
        .border_1()
        .border_color(theme.border)
        .rounded(px(10.0))
        .p(px(25.0))
        .overflow_hidden()
        .bg(theme.editorial_dev)
        .cursor_pointer()
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(px(6.0))
                        
                        .text_color(theme.muted_fg)
                        .child("FOR THE BUILDERS".to_string()),
                )
                .child(
                    div()
                        .mt(px(10.0))
                        .text_size(px(20.0))
                        .line_height(px(28.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.foreground)
                        .child("Less friction.\nMore creating.".to_string()),
                )
                .child(
                    div()
                        .mt(px(10.0))
                        .text_size(px(8.0))
                        .text_color(theme.muted_fg)
                        .child("Tools that keep you in your flow.".to_string()),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .mt(px(18.0))
                        .text_size(px(8.0))
                        .text_color(theme.foreground)
                        .child("Explore developer tools".to_string())
                        .child(icon("arrow-right").size(px(14.0)).text_color(theme.foreground)),
                ),
        )
        .child(
            div()
                .absolute()
                .right(px(-20.0))
                .bottom(px(25.0))
                .w(px(185.0))
                .p(px(14.0))
                .bg(theme.terminal_bg)
                .border_1()
                .border_color(theme.border)
                .rounded(px(8.0))
                .shadow_md()
                
                .child(
                    div()
                        .flex()
                        .gap(px(4.0))
                        .mb(px(17.0))
                        .child(div().size(px(4.0)).rounded_full().bg(theme.muted_fg))
                        .child(div().size(px(4.0)).rounded_full().bg(theme.muted_fg))
                        .child(div().size(px(4.0)).rounded_full().bg(theme.muted_fg)),
                )
                .child(
                    div()
                        .mt(px(8.0))
                        .font_family("monospace")
                        .text_size(px(8.0))
                        .text_color(theme.hero_fg)
                        .child(
                            div()
                                .flex()
                                .gap(px(6.0))
                                .child(
                                    div().text_color(theme.success).child("❯".to_string()),
                                )
                                .child("npm run dev".to_string()),
                        ),
                )
                .child(
                    div()
                        .mt(px(8.0))
                        .font_family("monospace")
                        .text_size(px(8.0))
                        .text_color(theme.muted_fg)
                        .child("ready in 248ms".to_string()),
                )
                .child(
                    div()
                        .mt(px(8.0))
                        .font_family("monospace")
                        .text_size(px(8.0))
                        .text_color(theme.hero_fg)
                        .child(
                            div()
                                .flex()
                                .gap(px(6.0))
                                .child(div().text_color(theme.success).child("✓".to_string()))
                                .child("Your next idea awaits_".to_string()),
                        ),
                ),
        )
        .id("editorial-dev").on_click(on_click)
}

fn editorial_focus_panel(theme: &Theme, on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Stateful<Div> {
    div()
        .relative()
        .flex()
        .items_center()
        .min_h(px(190.0))
        .flex_1()
        .border_1()
        .border_color(theme.border)
        .rounded(px(10.0))
        .p(px(25.0))
        .overflow_hidden()
        .bg(theme.editorial_focus)
        .cursor_pointer()
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(px(6.0))
                        
                        .text_color(theme.muted_fg)
                        .child("MAKE SPACE FOR WHAT MATTERS".to_string()),
                )
                .child(
                    div()
                        .mt(px(10.0))
                        .text_size(px(20.0))
                        .line_height(px(28.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.foreground)
                        .child("A more focused day.".to_string()),
                )
                .child(
                    div()
                        .mt(px(10.0))
                        .text_size(px(8.0))
                        .text_color(theme.muted_fg)
                        .child("Organize your work. Simplify your desktop.".to_string()),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .mt(px(18.0))
                        .text_size(px(8.0))
                        .text_color(theme.foreground)
                        .child("Explore productivity".to_string())
                        .child(icon("arrow-right").size(px(14.0)).text_color(theme.foreground)),
                ),
        )
        .child(
            div()
                .absolute()
                .right(px(-16.0))
                .bottom(px(26.0))
                .w(px(104.0))
                .h(px(125.0))
                .bg(theme.card)
                .border_1()
                .border_color(theme.border)
                .rounded(px(10.0))
                .p(px(20.0))
                .pt(px(20.0))
                .shadow_md()
                
                .child(div().h(px(5.0)).w(px(64.0)).rounded(px(3.0)).bg(theme.border).mb(px(8.0)))
                .child(div().h(px(5.0)).w(px(46.0)).rounded(px(3.0)).bg(theme.border).mb(px(8.0)))
                .child(div().h(px(5.0)).w(px(64.0)).rounded(px(3.0)).bg(theme.border).mb(px(8.0)))
                .child(
                    div()
                        .size(px(26.0))
                        .rounded_full()
                        .bg(theme.success)
                        .text_color(theme.card)
                        .flex()
                        .items_center()
                        .justify_center()
                        .mt(px(5.0))
                        .child(icon("check").size(px(14.0)).text_color(theme.card)),
                ),
        )
        .id("editorial-focus").on_click(on_click)
}
