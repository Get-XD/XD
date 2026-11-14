use gpui::{App, ClickEvent, Div, FontWeight, InteractiveElement, ParentElement, SharedString, Stateful, StatefulInteractiveElement, Styled, Transformation, Window, div, px, radians};
use gpui::prelude::{FluentBuilder};
use crate::icons::icon;
use crate::state::{StoreHandle, Toast, ToastKind};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Default,
    Sm,
    Icon,
}

pub fn button(
    theme: &Theme,
    variant: ButtonVariant,
    size: ButtonSize,
    label: Option<&str>,
    icon_name: Option<&str>,
    disabled: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    button_inner(
        theme,
        variant,
        size,
        label,
        icon_name,
        disabled,
        None,
        None,
        on_click,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn button_inner(
    theme: &Theme,
    variant: ButtonVariant,
    size: ButtonSize,
    label: Option<&str>,
    icon_name: Option<&str>,
    disabled: bool,
    height: Option<f32>,
    font_size: Option<f32>,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let h = height.unwrap_or(match size {
        ButtonSize::Default => 36.0,
        ButtonSize::Sm => 32.0,
        ButtonSize::Icon => 36.0,
    });
    let (bg, fg, border, hover_bg) = match variant {
        ButtonVariant::Default => (
            Some(theme.primary),
            Some(theme.primary_fg),
            None,
            Some(theme.primary.opacity(0.9)),
        ),
        ButtonVariant::Destructive => (
            Some(theme.destructive),
            Some(theme.primary_fg),
            None,
            Some(theme.destructive.opacity(0.9)),
        ),
        ButtonVariant::Outline => (
            Some(theme.background),
            Some(theme.foreground),
            Some(theme.border),
            Some(theme.accent),
        ),
        ButtonVariant::Secondary => (
            Some(theme.secondary),
            Some(theme.foreground),
            None,
            Some(theme.secondary.opacity(0.85)),
        ),
        ButtonVariant::Ghost => (None, Some(theme.muted_fg), None, Some(theme.accent)),
        ButtonVariant::Link => (None, Some(theme.primary), None, None),
    };

    let mut el = div()
        .flex()
        .items_center()
        .justify_center()
        .gap(px(8.0))
        .h(px(h))
        .px(match size {
            ButtonSize::Sm => px(12.0),
            ButtonSize::Icon => px(0.0),
            ButtonSize::Default => px(16.0),
        })
        .rounded_md()
        .text_size(px(font_size.unwrap_or(if size == ButtonSize::Sm {
            11.0
        } else {
            12.0
        })))
        .text_color(fg.unwrap_or(theme.foreground))
        .font_weight(FontWeight::MEDIUM)
        .whitespace_nowrap();

    if let Some(c) = bg {
        el = el.bg(c);
    }
    if let Some(c) = border {
        el = el.border_1().border_color(c);
    }

    if size == ButtonSize::Icon {
        el = el.w(px(h));
    }
    if variant == ButtonVariant::Link {
        el = el.underline();
    }
    if disabled {
        el = el.opacity(0.5);
    } else if let Some(hover) = hover_bg {
        el = el.hover(move |s| s.bg(hover));
    }

    if let Some(name) = icon_name {
        el = el.child(
            icon(name)
                .size(px(if size == ButtonSize::Sm { 11.0 } else { 14.0 }))
                .text_color(fg.unwrap_or(theme.foreground)),
        );
    }
    if let Some(text) = label {
        el = el.child(text.to_string());
    }

    let el = el.id(SharedString::from(format!("btn-{}", label.or(icon_name).unwrap_or("btn"))));
    if disabled {
        el
    } else {
        el.on_click(on_click)
    }
}

pub fn icon_button(
    theme: &Theme,
    icon_name: &str,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    div()
        .flex()
        .items_center()
        .justify_center()
        .size(px(36.0))
        .rounded_md()
        .text_color(theme.muted_fg)
        .hover(|s| s.bg(theme.accent).text_color(theme.foreground))
        .child(icon(icon_name).size(px(16.0)).text_color(theme.muted_fg))
        .id(SharedString::from(format!("ib-{}", icon_name))).on_click(on_click)
}

/// Simple select that cycles through options (mock-compatible).
pub fn select(
    theme: &Theme,
    value: &str,
    options: &[&str],
    width: Option<f32>,
    on_change: impl Fn(String, &mut App) + 'static,
) -> Stateful<Div> {
    let mut el = div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(8.0))
        .h(px(37.0))
        .px(px(12.0))
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.card)
        .text_size(px(11.0))
        .text_color(theme.foreground)
        .cursor_pointer()
        .child(value.to_string())
        .child(
            icon("chevron-right")
                .size(px(12.0))
                .text_color(theme.muted_fg)
                .with_transformation(Transformation::rotate(radians(std::f32::consts::FRAC_PI_2))),
        );
    el = if let Some(w) = width {
        el.w(px(w))
    } else {
        el.min_w(px(170.0))
    };

    let opts: Vec<String> = options.iter().map(|s| s.to_string()).collect();
    let current = value.to_string();
    el.id(SharedString::from(format!("sel-{}", current))).on_click(move |_, _, cx| {
        if let Some(ix) = opts.iter().position(|o| *o == current) {
            let next = opts[(ix + 1) % opts.len()].clone();
            on_change(next, cx);
        }
    })
}

/// Text field that displays value/placeholder; keystrokes routed via store focus.
pub fn text_field(
    theme: &Theme,
    value: &str,
    placeholder: &str,
    width: Option<f32>,
    focused: bool,
    on_focus: impl Fn(&mut App) + 'static,
) -> Stateful<Div> {
    let mut el = div()
        .id(SharedString::from(format!("tf-{placeholder}")))
        .flex()
        .items_center()
        .gap(px(10.0))
        .h(px(39.0))
        .px(px(12.0))
        .rounded_md()
        .border_1()
        .border_color(if focused { theme.primary } else { theme.border })
        .bg(theme.card)
        .text_size(px(12.0))
        .text_color(theme.foreground)
        .cursor_text();

    el = if let Some(w) = width {
        el.w(px(w))
    } else {
        el.flex_1()
    };

    if value.is_empty() && !focused {
        el = el.child(
            div()
                .text_size(px(12.0))
                .text_color(theme.muted_fg)
                .child(placeholder.to_string()),
        );
    } else {
        el = el.child(
            div()
                .flex_1()
                .min_w(px(0.0))
                .child(value.to_string())
                .when(focused, |d| {
                    d.child(
                        div()
                            .w(px(1.0))
                            .h(px(14.0))
                            .bg(theme.primary)
                            .ml(px(1.0)),
                    )
                }),
        );
    }

    el.on_click(move |_, _, cx| on_focus(cx))
}

pub fn switch_control(theme: &Theme, checked: bool, on_toggle: impl Fn(&mut App) + 'static) -> Stateful<Div> {
    let bg = if checked {
        theme.primary
    } else {
        theme.secondary
    };
    let knob = if checked {
        theme.primary_fg
    } else {
        theme.muted_fg
    };
    let knob_x = if checked { 15.0 } else { 3.0 };
    div()
        .flex()
        .items_center()
        .cursor_pointer()
        .child(
            div()
                .relative()
                .w(px(34.0))
                .h(px(19.0))
                .rounded(px(12.0))
                .bg(bg)
                .border_1()
                .border_color(if checked { theme.primary } else { theme.border })
                .child(
                    div()
                        .absolute()
                        .left(px(knob_x))
                        .top(px(3.0))
                        .size(px(13.0))
                        .rounded_full()
                        .bg(knob),
                ),
        )
        .id("switch").on_click(move |_, _, cx| on_toggle(cx))
}

pub fn progress_bar(theme: &Theme, value: u32, width: f32) -> Div {
    div()
        .w(px(width))
        .h(px(5.0))
        .rounded(px(4.0))
        .bg(theme.secondary)
        .child(
            div()
                .h_full()
                .w(px((value as f32 / 100.0) * width))
                .rounded(px(4.0))
                .bg(theme.primary),
        )
}

pub fn toasts_layer(theme: &Theme, toasts: &[Toast]) -> Div {
    let mut stack = div()
        .absolute()
        .bottom(px(20.0))
        .right(px(20.0))
        .flex()
        .flex_col()
        .gap(px(10.0))
        ;
    for t in toasts {
        let color = match t.kind {
            ToastKind::Success => theme.success,
            ToastKind::Error => theme.destructive,
            ToastKind::Info => theme.primary,
        };
        stack = stack.child(
            div()
                .flex()
                .items_start()
                .gap(px(10.0))
                .min_w(px(300.0))
                .max_w(px(360.0))
                .p(px(14.0))
                .rounded_md()
                .border_1()
                .border_color(theme.border)
                .bg(theme.popover)
                .shadow_lg()
                .child(div().size(px(8.0)).mt(px(4.0)).rounded_full().bg(color))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(3.0))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.foreground)
                                .child(t.title.clone()),
                        )
                        .when_some(t.description.clone(), |d, desc| {
                            d.child(div().text_size(px(11.0)).text_color(theme.muted_fg).child(desc))
                        }),
                ),
        );
    }
    stack
}

pub fn empty_state(theme: &Theme, title: &str, description: &str) -> Div {
    div()
        .flex()
        .flex_col()
        .items_center()
        .py(px(100.0))
        .px(px(20.0))
        .text_color(theme.muted_fg)
        .child(
            div()
                .mb(px(20.0))
                .child(icon("check").size(px(36.0)).text_color(theme.success)),
        )
        .child(
            div()
                .text_size(px(22.0))
                .text_color(theme.foreground)
                .mb(px(9.0))
                .child(title.to_string()),
        )
        .child(div().text_size(px(12.0)).child(description.to_string()))
}

pub fn section_heading(
    theme: &Theme,
    title: &str,
    description: Option<&str>,
    view_all: bool,
    on_view_all: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Div {
    let mut right = div();
    if view_all {
        right = right.child(
            button_inner(
                theme,
                ButtonVariant::Ghost,
                ButtonSize::Sm,
                Some("View all"),
                None,
                false,
                Some(28.0),
                Some(9.0),
                on_view_all,
            )
            .child(
                icon("arrow-up-right")
                    .size(px(12.0))
                    .text_color(theme.muted_fg),
            ),
        );
    }
    div()
        .flex()
        .items_center()
        .justify_between()
        .mb(px(13.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(5.0))
                .child(
                    div()
                        .text_size(px(16.0))
                        .font_weight(FontWeight(650.0))
                        .text_color(theme.foreground)
                        .child(title.to_string()),
                )
                .when_some(description, |d, desc| {
                    d.child(
                        div()
                            .text_size(px(10.0))
                            .text_color(theme.muted_fg)
                            .child(desc.to_string()),
                    )
                }),
        )
        .child(right)
}

pub fn page_intro(theme: &Theme, title: &str, text: &str, action: Option<Stateful<Div>>) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(15.0))
        .mb(px(30.0))
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(px(26.0))
                        .font_weight(FontWeight(650.0))
                        .text_color(theme.foreground)
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .mt(px(6.0))
                        .text_size(px(11.0))
                        .text_color(theme.muted_fg)
                        .child(text.to_string()),
                ),
        )
        .when_some(action, |d, a| d.child(a))
}

/// Modal panel (title + description + body + footer)
pub fn modal(theme: &Theme, title: &str, description: Option<&str>, width: f32, body: Div, footer: Option<Div>) -> Div {
    let mut panel = div()
        .flex()
        .flex_col()
        .w(px(width))
        .max_w(px(window_width_limit()))
        .bg(theme.popover)
        .border_1()
        .border_color(theme.border)
        .rounded_lg()
        .shadow_xl()
        .p(px(24.0))
        .gap(px(16.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .text_size(px(16.0))
                        .font_weight(FontWeight(650.0))
                        .text_color(theme.foreground)
                        .child(title.to_string()),
                )
                .when_some(description, |d, desc| {
                    d.child(div().text_size(px(12.0)).text_color(theme.muted_fg).child(desc.to_string()))
                }),
        )
        .child(body);
    if let Some(f) = footer {
        panel = panel.child(f);
    }
    div()
        .absolute()
        .inset_0()
        .flex()
        .items_center()
        .justify_center()
        .bg(theme.overlay)
        
        .child(panel)
}

fn window_width_limit() -> f32 {
    520.0
}

pub fn overlay_click_catcher(on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Stateful<Div> {
    div().absolute().inset_0().id("overlay-catcher").on_click(on_close)
}

pub fn store_read(store: &StoreHandle, cx: &App) -> std::sync::Arc<StoreStateSnapshot> {
    std::sync::Arc::new(StoreStateSnapshot::from(store.read(cx)))
}

pub struct StoreStateSnapshot {
    pub dummy: (),
}

impl StoreStateSnapshot {
    pub fn from(_s: &crate::state::StoreState) -> Self {
        Self { dummy: () }
    }
}
