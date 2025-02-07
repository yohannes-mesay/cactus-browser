pub mod paths;
pub mod prefs;

pub use paths::AppPaths;
pub use prefs::Preferences;

pub const APP_ID: &str = "org.cactus.Browser";
pub const APP_NAME: &str = "Cactus";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NEW_TAB_URI: &str = "cactus://newtab";
pub const HOME_URI: &str = "cactus://newtab";
