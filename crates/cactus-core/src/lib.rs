pub mod paths;
pub mod prefs;
pub mod store;
pub mod url_util;
pub mod internal;

pub use paths::AppPaths;
pub use prefs::Preferences;
pub use store::{Bookmark, DownloadRecord, HistoryEntry, Store};
pub use url_util::normalize_navigation_input;

pub const APP_ID: &str = "org.cactus.Browser";
pub const APP_NAME: &str = "Cactus";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NEW_TAB_URI: &str = "cactus://newtab";
pub const HOME_URI: &str = "cactus://newtab";
