mod about;
mod details;
mod downloads;
mod explore;
mod home;
mod settings;
mod updates;

pub use about::about_page;
pub use details::details_page;
pub use downloads::downloads_page;
pub use explore::{explore_page, installed_page};
pub use home::home_page;
pub use settings::settings_page;
pub use updates::updates_page;
