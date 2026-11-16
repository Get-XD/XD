use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Productivity,
    DeveloperTools,
    Utilities,
    Networking,
    FileManagement,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::Productivity => "Productivity",
            Category::DeveloperTools => "Developer Tools",
            Category::Utilities => "Utilities",
            Category::Networking => "Networking",
            Category::FileManagement => "File Management",
        }
    }

    pub fn all() -> &'static [&'static str] {
        &[
            "All",
            "Productivity",
            "Developer Tools",
            "Utilities",
            "Networking",
            "File Management",
        ]
    }

    pub fn from_label(s: &str) -> Option<Category> {
        match s {
            "Productivity" => Some(Category::Productivity),
            "Developer Tools" => Some(Category::DeveloperTools),
            "Utilities" => Some(Category::Utilities),
            "Networking" => Some(Category::Networking),
            "File Management" => Some(Category::FileManagement),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoreApp {
    pub id: String,
    pub name: String,
    pub tagline: String,
    pub description: String,
    pub category: Category,
    pub version: String,
    pub installed: Option<String>,
    pub size: u32,
    pub updated: String,
    pub added: String,
    pub color: String,
    pub icon: String,
    pub popular: u32,
    #[serde(default)]
    pub unavailable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Paused,
    Verifying,
    Installing,
    Completed,
    Failed,
    Cancelled,
}

impl DownloadStatus {
    pub fn label(&self) -> &'static str {
        match self {
            DownloadStatus::Queued => "Queued",
            DownloadStatus::Downloading => "Downloading",
            DownloadStatus::Paused => "Paused",
            DownloadStatus::Verifying => "Verifying",
            DownloadStatus::Installing => "Installing",
            DownloadStatus::Completed => "Completed",
            DownloadStatus::Failed => "Failed",
            DownloadStatus::Cancelled => "Cancelled",
        }
    }

    pub fn all() -> &'static [DownloadStatus] {
        use DownloadStatus::*;
        &[
            Queued,
            Downloading,
            Paused,
            Verifying,
            Installing,
            Completed,
            Failed,
            Cancelled,
        ]
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self,
            DownloadStatus::Downloading
                | DownloadStatus::Queued
                | DownloadStatus::Paused
                | DownloadStatus::Verifying
                | DownloadStatus::Installing
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Download {
    pub id: String,
    pub status: DownloadStatus,
    pub progress: u32,
}

#[derive(Clone, Debug)]
pub enum StoreAction {
    Install(String),
    Update(String),
    Uninstall(String),
    Ignore(String),
    Download(String, DownloadStatus),
    ClearCompleted,
}

pub fn catalog() -> Vec<StoreApp> {
    vec![
        StoreApp {
            id: "soundcore".into(),
            name: "Soundcore Desktop".into(),
            tagline: "Your sound. Your way.".into(),
            description:
                "Take control of your listening experience. Customize your sound, fine-tune your equalizer, and manage your Soundcore devices from one beautiful desktop app."
                    .into(),
            category: Category::Utilities,
            version: "2.4.0".into(),
            installed: Some("2.3.1".into()),
            size: 84,
            updated: "2026-11-12".into(),
            added: "2026-01-04".into(),
            color: "blue".into(),
            icon: "audio-lines".into(),
            popular: 98,
            unavailable: false,
        },
        StoreApp {
            id: "skybox".into(),
            name: "SkyBox".into(),
            tagline: "Your files, without boundaries.".into(),
            description:
                "A clean, effortless workspace for your files. Organize, sync, and access everything that matters.".into(),
            category: Category::FileManagement,
            version: "1.8.2".into(),
            installed: Some("1.8.2".into()),
            size: 62,
            updated: "2026-11-11".into(),
            added: "2026-02-11".into(),
            color: "cyan".into(),
            icon: "box".into(),
            popular: 94,
            unavailable: false,
        },
        StoreApp {
            id: "labscan".into(),
            name: "LabScan".into(),
            tagline: "Scan smarter. Work faster.".into(),
            description:
                "Turn documents into searchable, organized digital files with a powerful scanning workspace.".into(),
            category: Category::Productivity,
            version: "3.1.0".into(),
            installed: None,
            size: 128,
            updated: "2026-11-10".into(),
            added: "2026-04-02".into(),
            color: "purple".into(),
            icon: "scan-line".into(),
            popular: 90,
            unavailable: false,
        },
        StoreApp {
            id: "telegram".into(),
            name: "Telegram File Server".into(),
            tagline: "A new home for your files.".into(),
            description:
                "A lightweight file server powered by Telegram. Share and manage files securely from your desktop."
                    .into(),
            category: Category::Networking,
            version: "1.5.0".into(),
            installed: Some("1.4.2".into()),
            size: 46,
            updated: "2026-11-09".into(),
            added: "2026-03-08".into(),
            color: "teal".into(),
            icon: "send".into(),
            popular: 87,
            unavailable: false,
        },
        StoreApp {
            id: "devprocess".into(),
            name: "DevProcess".into(),
            tagline: "Less friction. More building.".into(),
            description:
                "Manage development processes, monitor ports, and keep your local environment running smoothly."
                    .into(),
            category: Category::DeveloperTools,
            version: "2.0.1".into(),
            installed: Some("1.9.0".into()),
            size: 38,
            updated: "2026-11-08".into(),
            added: "2026-05-21".into(),
            color: "orange".into(),
            icon: "terminal".into(),
            popular: 92,
            unavailable: false,
        },
        StoreApp {
            id: "clipnest".into(),
            name: "ClipNest".into(),
            tagline: "Never lose a good copy.".into(),
            description:
                "A thoughtful clipboard manager that keeps your snippets, links, and ideas within reach.".into(),
            category: Category::Productivity,
            version: "1.2.0".into(),
            installed: Some("1.2.0".into()),
            size: 24,
            updated: "2026-11-06".into(),
            added: "2026-07-18".into(),
            color: "green".into(),
            icon: "clipboard".into(),
            popular: 86,
            unavailable: false,
        },
        StoreApp {
            id: "portwatch".into(),
            name: "PortWatch".into(),
            tagline: "Know what is listening.".into(),
            description:
                "Inspect active network ports and connections with a focused developer-friendly interface.".into(),
            category: Category::DeveloperTools,
            version: "1.0.4".into(),
            installed: None,
            size: 19,
            updated: "2026-11-04".into(),
            added: "2026-08-20".into(),
            color: "rose".into(),
            icon: "network".into(),
            popular: 79,
            unavailable: false,
        },
        StoreApp {
            id: "focusflow".into(),
            name: "FocusFlow".into(),
            tagline: "Make room for deep work.".into(),
            description:
                "A minimal focus timer with sessions, breaks, and a distraction-free daily task list.".into(),
            category: Category::Productivity,
            version: "1.3.2".into(),
            installed: None,
            size: 32,
            updated: "2026-11-02".into(),
            added: "2026-09-01".into(),
            color: "yellow".into(),
            icon: "timer".into(),
            popular: 82,
            unavailable: false,
        },
    ]
}

pub fn initial_downloads() -> Vec<Download> {
    vec![
        Download {
            id: "labscan".into(),
            status: DownloadStatus::Downloading,
            progress: 64,
        },
        Download {
            id: "portwatch".into(),
            status: DownloadStatus::Queued,
            progress: 0,
        },
        Download {
            id: "focusflow".into(),
            status: DownloadStatus::Failed,
            progress: 28,
        },
        Download {
            id: "skybox".into(),
            status: DownloadStatus::Completed,
            progress: 100,
        },
    ]
}

pub fn apply_action(apps: &mut Vec<StoreApp>, downloads: &mut Vec<Download>, action: &StoreAction) {
    match action {
        StoreAction::ClearCompleted => {
            downloads.retain(|d| {
                !matches!(
                    d.status,
                    DownloadStatus::Completed | DownloadStatus::Cancelled
                )
            });
        }
        StoreAction::Ignore(_) => {}
        StoreAction::Download(id, status) => {
            if let Some(d) = downloads.iter_mut().find(|d| &d.id == id) {
                d.status = status.clone();
            }
        }
        StoreAction::Install(id) | StoreAction::Update(id) => {
            if let Some(app) = apps.iter_mut().find(|a| &a.id == id) {
                app.installed = Some(app.version.clone());
            }
            downloads.retain(|d| &d.id != id);
            downloads.insert(
                0,
                Download {
                    id: id.clone(),
                    status: DownloadStatus::Completed,
                    progress: 100,
                },
            );
        }
        StoreAction::Uninstall(id) => {
            if let Some(app) = apps.iter_mut().find(|a| &a.id == id) {
                app.installed = None;
            }
        }
    }
}

pub fn available_updates(apps: &[StoreApp], ignored: &[String]) -> Vec<StoreApp> {
    apps.iter()
        .filter(|a| {
            a.installed.is_some()
                && a.installed.as_deref() != Some(a.version.as_str())
                && !ignored.contains(&a.id)
        })
        .cloned()
        .collect()
}

pub fn filter_catalog(
    apps: &[StoreApp],
    search: &str,
    category: &str,
    sort: &str,
) -> Vec<StoreApp> {
    let search_l = search.to_lowercase();
    let mut filtered: Vec<StoreApp> = apps
        .iter()
        .filter(|a| {
            let cat_ok = category == "All" || a.category.label() == category;
            let text = format!("{} {}", a.name, a.description).to_lowercase();
            cat_ok && text.contains(&search_l)
        })
        .cloned()
        .collect();
    match sort {
        "Alphabetical" => filtered.sort_by(|a, b| a.name.cmp(&b.name)),
        "Recently Added" => filtered.sort_by(|a, b| b.added.cmp(&a.added)),
        _ => filtered.sort_by(|a, b| b.updated.cmp(&a.updated)),
    }
    filtered
}

pub fn repository(app: &StoreApp) -> String {
    format!(
        "https://github.com/Get-XD/{}",
        app.name.to_lowercase().replace(' ', "-")
    )
}
