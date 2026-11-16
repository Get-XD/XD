use gpui::{App, Entity};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::data::*;
use crate::theme::{Density, Theme};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    Home,
    Explore,
    Installed,
    Updates,
    Downloads,
    Details,
    Settings,
    About,
}

impl Route {
    pub fn path(&self) -> &'static str {
        match self {
            Route::Home => "/",
            Route::Explore => "/explore",
            Route::Installed => "/installed",
            Route::Updates => "/updates",
            Route::Downloads => "/downloads",
            Route::Details => "/apps",
            Route::Settings => "/settings",
            Route::About => "/about",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Route::Home => "Home",
            Route::Explore => "Explore",
            Route::Installed => "Installed",
            Route::Updates => "Updates",
            Route::Downloads => "Downloads",
            Route::Details => "Application details",
            Route::Settings => "Settings",
            Route::About => "About",
        }
    }

    pub fn nav_items() -> [(Route, &'static str, &'static str); 5] {
        [
            (Route::Home, "Home", "home"),
            (Route::Explore, "Explore", "compass"),
            (Route::Installed, "Installed", "layers"),
            (Route::Updates, "Updates", "refresh-cw"),
            (Route::Downloads, "Downloads", "download"),
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfirmationAction {
    Install,
    Update,
    Uninstall,
    UpdateAll,
}

#[derive(Clone, Debug)]
pub struct Confirmation {
    pub action: ConfirmationAction,
    pub app: Option<StoreApp>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Info,
    Error,
}

#[derive(Clone, Debug)]
pub struct Toast {
    pub title: String,
    pub description: Option<String>,
    pub kind: ToastKind,
    pub created: std::time::Instant,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialogKind {
    None,
    CommandPalette,
    Notifications,
    Confirmation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveInput {
    None,
    HeaderSearch,
    CommandSearch,
    ExploreSearch,
    InstalledSearch,
    DownloadDir,
    InstallDir,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub startup: bool,
    pub tray: bool,
    pub automatic: bool,
    pub notifications: bool,
    pub shortcuts: bool,
    pub confirmation: bool,
    pub directory: String,
    pub install_directory: String,
    pub concurrent: String,
    pub theme: String,
    pub density: String,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            startup: false,
            tray: true,
            automatic: true,
            notifications: true,
            shortcuts: true,
            confirmation: true,
            directory: Self::default_download_dir(),
            install_directory: r"C:\Program Files\XD".into(),
            concurrent: "3".into(),
            theme: "Dark".into(),
            density: "Comfortable".into(),
        }
    }
}

impl Preferences {
    const LEGACY_DOWNLOAD_DIR: &'static str = r"C:\Users\XD\Downloads";

    pub fn default_download_dir() -> String {
        std::env::var("USERPROFILE")
            .ok()
            .map(|u| u.trim_end_matches('\\').to_string())
            .filter(|u| !u.is_empty())
            .map(|u| format!(r"{}\Downloads", u))
            .unwrap_or_else(|| Self::LEGACY_DOWNLOAD_DIR.into())
    }

    pub fn file_path() -> PathBuf {
        let base = std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        base.join("XD").join("preferences.json")
    }

    pub fn load() -> Preferences {
        let mut prefs: Preferences = std::fs::read_to_string(Self::file_path())
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default();
        if prefs.directory == Self::LEGACY_DOWNLOAD_DIR {
            prefs.directory = Self::default_download_dir();
            prefs.save();
        }
        prefs
    }

    pub fn save(&self) {
        let path = Self::file_path();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, json);
        }
    }
}

pub struct StoreState {
    pub apps: Vec<StoreApp>,
    pub downloads: Vec<Download>,
    pub ignored: Vec<String>,
    pub preferences: Preferences,
    pub route: Route,
    pub detail_app_id: Option<String>,
    pub explore_category: String,
    pub sidebar_collapsed: bool,
    pub dialog: DialogKind,
    pub confirmation: Option<Confirmation>,
    pub keep_data: bool,
    pub command_query: String,
    pub explore_search: String,
    pub explore_sort: String,
    pub explore_list: bool,
    pub installed_search: String,
    pub installed_filter: String,
    pub details_tab: String,
    pub details_shot: usize,
    pub toasts: Vec<Toast>,
    pub theme: Theme,
    pub active_input: ActiveInput,
    pub pending_installs: Vec<String>,
    pub persisted_prefs: Preferences,
}

impl StoreState {
    pub fn new() -> Self {
        let preferences = Preferences::load();
        let persisted_prefs = preferences.clone();
        Self {
            apps: catalog(),
            downloads: initial_downloads(),
            ignored: Vec::new(),
            preferences,
            persisted_prefs,
            pending_installs: Vec::new(),
            route: Route::Home,
            detail_app_id: None,
            explore_category: "All".into(),
            sidebar_collapsed: false,
            dialog: DialogKind::None,
            confirmation: None,
            keep_data: true,
            command_query: String::new(),
            explore_search: String::new(),
            explore_sort: "Recently Updated".into(),
            explore_list: false,
            installed_search: String::new(),
            installed_filter: "All installed".into(),
            details_tab: "Overview".into(),
            details_shot: 0,
            toasts: Vec::new(),
            theme: Theme::dark(),
            active_input: ActiveInput::None,
        }
    }

    pub fn focus_input(&mut self, input: ActiveInput) {
        self.active_input = input;
        if input == ActiveInput::HeaderSearch || input == ActiveInput::CommandSearch {
            self.dialog = DialogKind::CommandPalette;
        }
    }

    pub fn blur_input(&mut self) {
        self.active_input = ActiveInput::None;
    }

    pub fn input_value(&self) -> &str {
        match self.active_input {
            ActiveInput::HeaderSearch | ActiveInput::CommandSearch => &self.command_query,
            ActiveInput::ExploreSearch => &self.explore_search,
            ActiveInput::InstalledSearch => &self.installed_search,
            ActiveInput::DownloadDir => &self.preferences.directory,
            ActiveInput::InstallDir => &self.preferences.install_directory,
            ActiveInput::None => "",
        }
    }

    pub fn type_char(&mut self, ch: char) {
        match self.active_input {
            ActiveInput::HeaderSearch | ActiveInput::CommandSearch => self.command_query.push(ch),
            ActiveInput::ExploreSearch => self.explore_search.push(ch),
            ActiveInput::InstalledSearch => self.installed_search.push(ch),
            ActiveInput::DownloadDir => self.preferences.directory.push(ch),
            ActiveInput::InstallDir => self.preferences.install_directory.push(ch),
            ActiveInput::None => {}
        }
    }

    pub fn backspace(&mut self) {
        match self.active_input {
            ActiveInput::HeaderSearch | ActiveInput::CommandSearch => {
                self.command_query.pop();
            }
            ActiveInput::ExploreSearch => {
                self.explore_search.pop();
            }
            ActiveInput::InstalledSearch => {
                self.installed_search.pop();
            }
            ActiveInput::DownloadDir => {
                self.preferences.directory.pop();
            }
            ActiveInput::InstallDir => {
                self.preferences.install_directory.pop();
            }
            ActiveInput::None => {}
        }
    }

    pub fn apply_theme(&mut self) {
        let mut theme = if self.preferences.theme == "Light" {
            Theme::light()
        } else {
            Theme::dark()
        };
        theme.density = if self.preferences.density == "Compact" {
            Density::Compact
        } else {
            Density::Comfortable
        };
        self.theme = theme;
    }

    pub fn navigate(&mut self, route: Route, app_id: Option<String>) {
        self.route = route;
        if route == Route::Details {
            self.detail_app_id = app_id;
            self.details_tab = "Overview".into();
            self.details_shot = 0;
        }
        if route == Route::Explore {
            // keep filters
        }
        self.dialog = DialogKind::None;
    }

    pub fn updates(&self) -> Vec<StoreApp> {
        available_updates(&self.apps, &self.ignored)
    }

    pub fn active_downloads(&self) -> usize {
        self.downloads
            .iter()
            .filter(|d| d.status.is_active())
            .count()
    }

    pub fn dispatch(&mut self, action: StoreAction) {
        if let StoreAction::Ignore(id) = &action {
            if !self.ignored.contains(id) {
                self.ignored.push(id.clone());
            }
            return;
        }
        apply_action(&mut self.apps, &mut self.downloads, &action);
    }

    pub fn toast(&mut self, title: impl Into<String>, description: Option<String>, kind: ToastKind) {
        self.toasts.push(Toast {
            title: title.into(),
            description,
            kind,
            created: std::time::Instant::now(),
        });
        if self.toasts.len() > 4 {
            self.toasts.remove(0);
        }
    }

    pub fn find_app(&self, id: &str) -> Option<&StoreApp> {
        self.apps.iter().find(|a| a.id == id)
    }

    pub fn begin_download(&mut self, id: &str) {
        if self.pending_installs.iter().any(|x| x == id) {
            return;
        }
        self.downloads.retain(|d| d.id != id);
        self.downloads.insert(0, Download::queued(id));
        self.pending_installs.push(id.to_string());
    }

    pub fn tick_due(&self) -> bool {
        if self
            .toasts
            .iter()
            .any(|t| t.created.elapsed() >= std::time::Duration::from_millis(4500))
        {
            return true;
        }
        if self.preferences != self.persisted_prefs {
            return true;
        }
        self.pending_installs.iter().any(|id| {
            self.downloads.iter().any(|d| {
                d.id == *id
                    && matches!(
                        d.status,
                        DownloadStatus::Queued
                            | DownloadStatus::Downloading
                            | DownloadStatus::Verifying
                            | DownloadStatus::Installing
                    )
            })
        })
    }

    pub fn tick_downloads(&mut self) {
        let max = self
            .preferences
            .concurrent
            .parse::<usize>()
            .unwrap_or(3)
            .max(1);
        let mut active = self
            .downloads
            .iter()
            .filter(|d| {
                self.pending_installs.contains(&d.id)
                    && matches!(
                        d.status,
                        DownloadStatus::Downloading
                            | DownloadStatus::Verifying
                            | DownloadStatus::Installing
                    )
            })
            .count();
        for d in &mut self.downloads {
            if active >= max {
                break;
            }
            if d.status == DownloadStatus::Queued && self.pending_installs.contains(&d.id) {
                d.status = DownloadStatus::Downloading;
                active += 1;
            }
        }
        let mut finished: Vec<String> = Vec::new();
        for d in &mut self.downloads {
            if !self.pending_installs.contains(&d.id) {
                continue;
            }
            match d.status {
                DownloadStatus::Downloading => {
                    let step = 4
                        + (d.id.bytes().map(|b| b as usize).sum::<usize>() % 9);
                    d.progress = (d.progress + step as u32).min(100);
                    if d.progress >= 100 {
                        d.status = DownloadStatus::Verifying;
                    }
                }
                DownloadStatus::Verifying => {
                    d.status = DownloadStatus::Installing;
                }
                DownloadStatus::Installing => {
                    finished.push(d.id.clone());
                }
                _ => {}
            }
        }
        for id in finished {
            self.pending_installs.retain(|x| *x != id);
            let (name, is_update) = match self.find_app(&id) {
                Some(a) => (a.name.clone(), a.installed.is_some()),
                None => (id.clone(), false),
            };
            let action = if is_update {
                StoreAction::Update(id.clone())
            } else {
                StoreAction::Install(id.clone())
            };
            self.dispatch(action);
            self.toast(
                if is_update {
                    format!("{name} updated")
                } else {
                    format!("{name} installed")
                },
                Some("Simulated download complete.".to_string()),
                ToastKind::Success,
            );
        }
    }

    pub fn prune_toasts(&mut self) {
        self.toasts
            .retain(|t| t.created.elapsed() < std::time::Duration::from_millis(4500));
    }

    pub fn sync_prefs(&mut self) {
        if self.preferences != self.persisted_prefs {
            self.preferences.save();
            self.persisted_prefs = self.preferences.clone();
        }
    }
}

pub type StoreHandle = Entity<StoreState>;

pub fn theme_of(cx: &App, store: &StoreHandle) -> Theme {
    store.read(cx).theme.clone()
}
