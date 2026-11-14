use crate::icons::icon;
use crate::state::{ActiveInput, Route, StoreHandle, ToastKind};
use crate::theme::Theme;
use crate::ui::*;
use gpui::{App, Div, FontWeight, IntoElement, ParentElement, Styled, div, px};

pub fn settings_page(theme: Theme, store: StoreHandle, cx: &mut App) -> Div {
    let prefs = store.read(cx).preferences.clone();
    let dir_focused = store.read(cx).active_input == ActiveInput::DownloadDir;
    let install_focused = store.read(cx).active_input == ActiveInput::InstallDir;

    let mut root = div().flex().flex_col().max_w(px(900.0));

    root = root.child(page_intro(
        &theme,
        "Make yourself at home.",
        "Your store, set up the way you like it. Preferences apply to this session.",
        None,
    ));

    // General
    root = root.child(section_header(&theme, "settings-2", "General"));
    for (name, desc, key) in [
        (
            "Launch at startup",
            "Open Pamod Store when you sign in to your computer.",
            "startup",
        ),
        (
            "Minimize to tray",
            "Keep the store within reach when you close the window.",
            "tray",
        ),
        (
            "Check for updates automatically",
            "Check for new application versions in the background.",
            "automatic",
        ),
        (
            "Update notifications",
            "Get notified when updates are available.",
            "notifications",
        ),
    ] {
        let checked = match key {
            "startup" => prefs.startup,
            "tray" => prefs.tray,
            "automatic" => prefs.automatic,
            _ => prefs.notifications,
        };
        let s = store.clone();
        root = root.child(setting_row(
            &theme,
            name,
            desc,
            switch_control(&theme, checked, move |cx| {
                s.update(cx, |st, _| match key {
                    "startup" => st.preferences.startup = !st.preferences.startup,
                    "tray" => st.preferences.tray = !st.preferences.tray,
                    "automatic" => st.preferences.automatic = !st.preferences.automatic,
                    _ => st.preferences.notifications = !st.preferences.notifications,
                });
                cx.notify(s.entity_id());
            }),
        ));
    }

    // Downloads
    root = root.child(section_header(&theme, "download", "Downloads"));
    let s_dir = store.clone();
    root = root.child(setting_row(
        &theme,
        "Download directory",
        "Where downloaded packages will be saved.",
        text_field(
            &theme,
            &prefs.directory,
            "",
            Some(270.0),
            dir_focused,
            move |cx| {
                s_dir.update(cx, |st, _| {
                    st.focus_input(ActiveInput::DownloadDir);
                });
                cx.notify(s_dir.entity_id());
            },
        ),
    ));
    let s_conc = store.clone();
    root = root.child(setting_row(
        &theme,
        "Concurrent downloads",
        "Maximum number of simultaneous downloads.",
        select(
            &theme,
            &prefs.concurrent,
            &["1", "2", "3", "4", "5"],
            Some(100.0),
            move |v, cx| {
                s_conc.update(cx, |st, _| st.preferences.concurrent = v);
                cx.notify(s_conc.entity_id());
            },
        ),
    ));
    let s_cache = store.clone();
    root = root.child(setting_row(
        &theme,
        "Download cache",
        "Clear temporary packages from the demo cache.",
        button(
            &theme,
            ButtonVariant::Outline,
            ButtonSize::Sm,
            Some("Clear cache"),
            Some("trash-2"),
            false,
            move |_, _, cx| {
                s_cache.update(cx, |st, _| {
                    st.toast(
                        "Demo cache cleared",
                        Some("No local files were modified.".into()),
                        ToastKind::Success,
                    );
                });
                cx.notify(s_cache.entity_id());
            },
        ),
    ));

    // Installation
    root = root.child(section_header(&theme, "package-open", "Installation"));
    let s_inst = store.clone();
    root = root.child(setting_row(
        &theme,
        "Default installation directory",
        "The default home for your applications.",
        text_field(
            &theme,
            &prefs.install_directory,
            "",
            Some(270.0),
            install_focused,
            move |cx| {
                s_inst.update(cx, |st, _| {
                    st.focus_input(ActiveInput::InstallDir);
                });
                cx.notify(s_inst.entity_id());
            },
        ),
    ));
    for (name, desc, key) in [
        (
            "Create desktop shortcuts",
            "Add shortcuts for newly installed applications.",
            "shortcuts",
        ),
        (
            "Installation confirmation",
            "Review application information before installing.",
            "confirmation",
        ),
    ] {
        let checked = if key == "shortcuts" {
            prefs.shortcuts
        } else {
            prefs.confirmation
        };
        let s = store.clone();
        root = root.child(setting_row(
            &theme,
            name,
            desc,
            switch_control(&theme, checked, move |cx| {
                s.update(cx, |st, _| match key {
                    "shortcuts" => st.preferences.shortcuts = !st.preferences.shortcuts,
                    _ => st.preferences.confirmation = !st.preferences.confirmation,
                });
                cx.notify(s.entity_id());
            }),
        ));
    }

    // Appearance
    root = root.child(section_header(&theme, "palette", "Appearance"));
    let s_theme = store.clone();
    root = root.child(setting_row(
        &theme,
        "Theme",
        "Choose your preferred appearance.",
        select(
            &theme,
            &prefs.theme,
            &["Dark", "Light", "System"],
            Some(140.0),
            move |v, cx| {
                s_theme.update(cx, |st, _| {
                    st.preferences.theme = v;
                    st.apply_theme();
                });
                cx.notify(s_theme.entity_id());
            },
        ),
    ));
    let s_dens = store.clone();
    root = root.child(setting_row(
        &theme,
        "Interface density",
        "A little more room, or a little more information.",
        select(
            &theme,
            &prefs.density,
            &["Comfortable", "Compact"],
            Some(160.0),
            move |v, cx| {
                s_dens.update(cx, |st, _| {
                    st.preferences.density = v;
                    st.apply_theme();
                });
                cx.notify(s_dens.entity_id());
            },
        ),
    ));

    // About
    root = root.child(section_header(&theme, "package-open", "About"));
    let s_chk = store.clone();
    root = root.child(setting_row(
        &theme,
        "Pamod Store v1.0.0",
        "Developed by Pamod. Desktop software, thoughtfully made.",
        button(
            &theme,
            ButtonVariant::Outline,
            ButtonSize::Sm,
            Some("Check for store updates"),
            Some("refresh-cw"),
            false,
            move |_, _, cx| {
                s_chk.update(cx, |st, _| {
                    st.toast(
                        "You are using the latest demo version: v1.0.0",
                        None,
                        ToastKind::Info,
                    );
                });
                cx.notify(s_chk.entity_id());
            },
        ),
    ));
    let s_about = store.clone();
    root = root.child(
        div().mt(px(16.0)).child(button(
            &theme,
            ButtonVariant::Link,
            ButtonSize::Default,
            Some("More about Pamod Store"),
            Some("arrow-up-right"),
            false,
            move |_, _, cx| {
                s_about.update(cx, |s, _| s.navigate(Route::About, None));
                cx.notify(s_about.entity_id());
            },
        )),
    );

    root
}

fn section_header(theme: &Theme, icon_name: &str, title: &str) -> Div {
    div()
        .mt(px(35.0))
        .mb(px(0.0))
        .flex()
        .items_center()
        .gap(px(10.0))
        .pb(px(15.0))
        .border_b(px(1.0))
        .border_color(theme.border)
        .text_size(px(15.0))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.foreground)
        .child(icon(icon_name).size(px(17.0)).text_color(theme.primary))
        .child(title.to_string())
}

fn setting_row(theme: &Theme, name: &str, desc: &str, control: impl IntoElement) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(20.0))
        .py(px(19.0))
        .border_b(px(1.0))
        .border_color(theme.border)
        .child(
            div()
                .flex_1()
                .min_w(px(0.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.foreground)
                        .child(name.to_string()),
                )
                .child(
                    div()
                        .mt(px(5.0))
                        .text_size(px(10.0))
                        .line_height(px(16.0))
                        .text_color(theme.muted_fg)
                        .child(desc.to_string()),
                ),
        )
        .child(control)
}
