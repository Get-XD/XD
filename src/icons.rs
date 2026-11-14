use gpui::{IntoElement, SharedString, Styled, Svg, px, svg};

pub fn icon(name: &str) -> Svg {
    svg().path(format!("icons/{name}.svg"))
}

pub fn icon_named(name: &str, size: f32) -> impl IntoElement {
    svg().path(format!("icons/{name}.svg")).size(px(size))
}

pub fn path_asset(path: impl Into<SharedString>) -> Svg {
    svg().path(path)
}
