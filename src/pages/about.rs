use crate::icons::icon;
use crate::state::StoreHandle;
use crate::theme::Theme;
use crate::ui::*;
use gpui::{App, Div, FontWeight, ParentElement, Styled, div, px};

pub fn about_page(theme: Theme, store: StoreHandle, _cx: &mut App) -> Div {
    let s_gh = store.clone();
    let s_src = store.clone();

    div()
        .flex()
        .flex_col()
        .items_center()
        .max_w(px(650.0))
        .mx_auto()
        .my(px(50.0))
        .text_center()
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .w(px(100.0))
                .h(px(100.0))
                .rounded(px(25.0))
                .bg(theme.primary)
                .text_color(theme.primary_fg)
                .shadow_md()
                .mb(px(30.0))
                .child(icon("package-open").size(px(48.0)).text_color(theme.primary_fg)),
        )
        .child(
            div()
                .text_size(px(8.0))
                
                .font_weight(FontWeight::BOLD)
                .text_color(theme.muted_fg)
                .child("YOUR PERSONAL SOFTWARE HUB".to_string()),
        )
        .child(
            div()
                .mt(px(10.0))
                .text_size(px(38.0))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.foreground)
                .child("Pamod Store".to_string()),
        )
        .child(
            div()
                .mt(px(12.0))
                .flex()
                .justify_center()
                .gap(px(8.0))
                .text_size(px(11.0))
                .text_color(theme.muted_fg)
                .child("Version 1.0.0".to_string())
                .child("·".to_string())
                .child("Desktop prototype".to_string()),
        )
        .child(
            div()
                .mt(px(30.0))
                .text_size(px(16.0))
                .text_color(theme.foreground)
                .child("Thoughtfully built software. One place to call home.".to_string()),
        )
        .child(
            div()
                .mt(px(18.0))
                .max_w(px(460.0))
                .text_size(px(12.0))
                .line_height(px(22.8))
                .text_color(theme.muted_fg)
                .child(
                    "A personal collection of desktop applications developed by Pamod. Discover new tools, keep your favorites up to date, and bring a little more ease to your everyday workflow."
                        .to_string(),
                ),
        )
        .child(
            div()
                .flex()
                .justify_center()
                .gap(px(12.0))
                .mt(px(30.0))
                .child(button(
                    &theme,
                    ButtonVariant::Outline,
                    ButtonSize::Default,
                    Some("GitHub profile"),
                    Some("github"),
                    false,
                    move |_, _, cx| {
                        s_gh.update(cx, |s, _| {
                            s.toast(
                                "GitHub profile",
                                Some("https://github.com/PamodMadubashana".into()),
                                crate::state::ToastKind::Info,
                            );
                        });
                        cx.notify(s_gh.entity_id());
                    },
                ))
                .child(button(
                    &theme,
                    ButtonVariant::Outline,
                    ButtonSize::Default,
                    Some("Source code"),
                    Some("package-open"),
                    false,
                    move |_, _, cx| {
                        s_src.update(cx, |s, _| {
                            s.toast(
                                "Source code",
                                Some(
                                    "https://github.com/PamodMadubashana/pamod-store".into(),
                                ),
                                crate::state::ToastKind::Info,
                            );
                        });
                        cx.notify(s_src.entity_id());
                    },
                )),
        )
        .child(
            div()
                .flex()
                .justify_center()
                .gap(px(55.0))
                .py(px(30.0))
                .my(px(30.0))
                .border_t(px(1.0))
                .border_b(px(1.0))
                .border_color(theme.border)
                .child(fact(&theme, "DEVELOPER", "Pamod"))
                .child(fact(&theme, "PLATFORM", "Windows 10 / 11"))
                .child(fact(&theme, "LICENSE", "MIT (sample)")),
        )
        .child(
            div()
                .flex()
                .items_start()
                .gap(px(12.0))
                .mt(px(28.0))
                .p(px(18.0))
                .bg(theme.card)
                .border_1()
                .border_color(theme.border)
                .rounded(px(8.0))
                .text_left()
                .text_color(theme.muted_fg)
                .child(
                    icon("shield-check")
                        .size(px(20.0))
                        .flex_shrink_0()
                        .text_color(theme.primary),
                )
                .child(
                    div()
                        .text_size(px(10.0))
                        .line_height(px(18.0))
                        .child(
                            "This is a local interface prototype. Installations, updates, and downloads are demonstrations; no software or system files are modified. Repository links and license details are illustrative."
                                .to_string(),
                        ),
                ),
        )
}

fn fact(theme: &Theme, label: &str, value: &str) -> Div {
    div()
        .flex()
        .flex_col()
        .text_center()
        .child(
            div()
                .text_size(px(8.0))
                
                .text_color(theme.muted_fg)
                .mb(px(8.0))
                .child(label.to_string()),
        )
        .child(
            div()
                .text_size(px(12.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.foreground)
                .child(value.to_string()),
        )
}
