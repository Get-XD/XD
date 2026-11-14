use gpui::{Hsla, rgb};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Density {
    Comfortable,
    Compact,
}

#[derive(Clone)]
pub struct Theme {
    pub mode: ThemeMode,
    pub density: Density,
    pub background: Hsla,
    pub foreground: Hsla,
    pub card: Hsla,
    pub popover: Hsla,
    pub primary: Hsla,
    pub primary_fg: Hsla,
    pub secondary: Hsla,
    pub muted: Hsla,
    pub muted_fg: Hsla,
    pub accent: Hsla,
    pub border: Hsla,
    pub destructive: Hsla,
    pub success: Hsla,
    pub sidebar: Hsla,
    pub overlay: Hsla,
    pub hero_fg: Hsla,
    pub hero_muted: Hsla,
    pub hero_shade: Hsla,
    pub blue_soft: Hsla,
    pub icon_blue: Hsla,
    pub icon_cyan: Hsla,
    pub icon_purple: Hsla,
    pub icon_teal: Hsla,
    pub icon_orange: Hsla,
    pub icon_green: Hsla,
    pub icon_rose: Hsla,
    pub icon_yellow: Hsla,
    pub terminal_bg: Hsla,
    pub editorial_dev: Hsla,
    pub editorial_focus: Hsla,
}

fn hx(s: &str) -> Hsla {
    let s = s.trim_start_matches('#');
    let r = u8::from_str_radix(&s[0..2], 16).unwrap() as f32 / 255.0;
    let g = u8::from_str_radix(&s[2..4], 16).unwrap() as f32 / 255.0;
    let b = u8::from_str_radix(&s[4..6], 16).unwrap() as f32 / 255.0;
    rgb(((r * 255.0) as u32) << 16 | ((g * 255.0) as u32) << 8 | (b * 255.0) as u32).into()
}

fn hx_a(s: &str, a: f32) -> Hsla {
    let mut c = hx(s);
    c.a = a;
    c
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            density: Density::Comfortable,
            background: hx("#0c0d0f"),
            foreground: hx("#ebedf0"),
            card: hx("#131518"),
            popover: hx("#18191d"),
            primary: hx("#2884ff"),
            primary_fg: hx("#fcfcfc"),
            secondary: hx("#222429"),
            muted: hx("#191a1e"),
            muted_fg: hx("#888a92"),
            accent: hx("#21252d"),
            border: hx("#2b2c31"),
            destructive: hx("#f66c6b"),
            success: hx("#2cc48f"),
            sidebar: hx("#101114"),
            overlay: hx_a("#010202", 0.8),
            hero_fg: hx("#f4f5f7"),
            hero_muted: hx("#a9b2bf"),
            hero_shade: hx("#04070f"),
            blue_soft: hx("#091a34"),
            icon_blue: hx("#2381ff"),
            icon_cyan: hx("#00b6e6"),
            icon_purple: hx("#9065f0"),
            icon_teal: hx("#00bccf"),
            icon_orange: hx("#f58028"),
            icon_green: hx("#2bb985"),
            icon_rose: hx("#e9609e"),
            icon_yellow: hx("#e3ae28"),
            terminal_bg: hx("#090b11"),
            editorial_dev: hx("#16161e"),
            editorial_focus: hx("#111a15"),
        }
    }

    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            density: Density::Comfortable,
            background: hx("#f4f5f6"),
            foreground: hx("#1b1d23"),
            card: hx("#ffffff"),
            popover: hx("#ffffff"),
            primary: hx("#2884ff"),
            primary_fg: hx("#fcfcfc"),
            secondary: hx("#dde1e8"),
            muted: hx("#e6e8eb"),
            muted_fg: hx("#5e6066"),
            accent: hx("#d8e2f2"),
            border: hx("#cfd1d6"),
            destructive: hx("#f66c6b"),
            success: hx("#2cc48f"),
            sidebar: hx("#eceff2"),
            overlay: hx_a("#010202", 0.8),
            hero_fg: hx("#f4f5f7"),
            hero_muted: hx("#a9b2bf"),
            hero_shade: hx("#04070f"),
            blue_soft: hx("#d1e3fc"),
            icon_blue: hx("#2381ff"),
            icon_cyan: hx("#00b6e6"),
            icon_purple: hx("#9065f0"),
            icon_teal: hx("#00bccf"),
            icon_orange: hx("#f58028"),
            icon_green: hx("#2bb985"),
            icon_rose: hx("#e9609e"),
            icon_yellow: hx("#e3ae28"),
            terminal_bg: hx("#090b11"),
            editorial_dev: hx("#e2e4ef"),
            editorial_focus: hx("#dce8e2"),
        }
    }

    pub fn icon_color(&self, name: &str) -> Hsla {
        match name {
            "cyan" => self.icon_cyan,
            "purple" => self.icon_purple,
            "teal" => self.icon_teal,
            "orange" => self.icon_orange,
            "green" => self.icon_green,
            "rose" => self.icon_rose,
            "yellow" => self.icon_yellow,
            _ => self.icon_blue,
        }
    }

    pub fn with_density(mut self, density: Density) -> Self {
        self.density = density;
        self
    }

    pub fn content_pad_top(&self) -> f32 {
        if self.density == Density::Compact {
            20.0
        } else {
            31.0
        }
    }

    pub fn section_gap(&self) -> f32 {
        if self.density == Density::Compact {
            20.0
        } else {
            28.0
        }
    }
}

pub fn lighten(c: Hsla, amount: f32) -> Hsla {
    Hsla {
        l: (c.l + (1.0 - c.l) * amount).min(1.0),
        s: c.s * (1.0 - amount),
        ..c
    }
}

pub fn darken(c: Hsla, amount: f32) -> Hsla {
    Hsla {
        l: c.l * (1.0 - amount),
        s: c.s * (1.0 - amount),
        ..c
    }
}
