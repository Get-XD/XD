mod components;
mod data;
mod icons;
mod pages;
mod shell;
mod state;
mod theme;
mod ui;

use gpui::{TitlebarOptions, WindowOptions, App, AppContext, Application, AssetSource, Bounds, SharedString, WindowBounds, px, size};
use state::StoreState;
use std::borrow::Cow;
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        let full = self.base.join(path);
        std::fs::read(full)
            .map(|data| Some(Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let full = self.base.join(path);
        std::fs::read_dir(full)
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    Application::new()
        .with_assets(Assets {
            base: base.clone(),
        })
        .run(move |cx: &mut App| {
            // Load Manrope
            if let Ok(font) = std::fs::read(base.join("fonts/Manrope.ttf")) {
                let _ = cx
                    .text_system()
                    .add_fonts(vec![Cow::Owned(font)]);
            }

            let store = cx.new(|_| StoreState::new());
            store.update(cx, |s, _| s.apply_theme());

            let bounds = Bounds::centered(
                None,
                size(px(1360.0), px(900.0)),
                cx,
            );
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("XD Store".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        cx.observe(&store, |_, _, cx| cx.notify()).detach();
                        shell::StoreShell::new(store.clone())
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
}
