pub mod bookmarks_io;
pub mod cli;
pub mod errors;
pub mod internal;
pub mod metrics;
pub mod omnibox;
pub mod paths;
pub mod permissions;
pub mod project;
pub mod prefs;
pub mod sanitize;
pub mod search;
pub mod store;
pub mod url_util;

pub use cli::{help_text, LaunchArgs};
pub use errors::{CactusError, CactusResult};
pub use omnibox::{suggest, Suggestion, SuggestionKind};
pub use paths::AppPaths;
pub use permissions::{origin_from_url, PermissionDecision, PermissionKind, SitePermission};
pub use prefs::{encode_query, Preferences, SearchEngine};
pub use store::{Bookmark, DownloadRecord, HistoryEntry, SessionState, SessionWindow, Store};
pub use url_util::normalize_navigation_input;

pub const APP_ID: &str = "org.cactus.Browser";
pub const APP_NAME: &str = "Cactus";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NEW_TAB_URI: &str = "cactus://newtab";
pub const HOME_URI: &str = "cactus://newtab";

pub use project::{GITHUB_DOCS_README, GITHUB_ISSUES, GITHUB_RELEASES, GITHUB_REPO, HOMEPAGE};
