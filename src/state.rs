use gpui::{App, Entity};
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

#[derive(Clone)]
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
            directory: r"C:\Users\Pamod\Downloads".into(),
            install_directory: r"C:\Program Files\Pamod".into(),
            concurrent: "3".into(),
            theme: "Dark".into(),
            density: "Comfortable".into(),
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
}

impl StoreState {
    pub fn new() -> Self {
        Self {
            apps: catalog(),
            downloads: initial_downloads(),
            ignored: Vec::new(),
            preferences: Preferences::default(),
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
        });
        if self.toasts.len() > 4 {
            self.toasts.remove(0);
        }
    }

    pub fn find_app(&self, id: &str) -> Option<&StoreApp> {
        self.apps.iter().find(|a| a.id == id)
    }
}

pub type StoreHandle = Entity<StoreState>;

pub fn theme_of(cx: &App, store: &StoreHandle) -> Theme {
    store.read(cx).theme.clone()
}
