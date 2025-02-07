use crate::paths::AppPaths;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub homepage: String,
    pub search_engine: SearchEngine,
    pub restore_session: bool,
    pub https_first: bool,
    pub block_ads_trackers: bool,
    pub send_dnt: bool,
    pub send_gpc: bool,
    pub show_bookmark_bar: bool,
    pub default_zoom: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SearchEngine {
    DuckDuckGo,
    Startpage,
    Wikipedia,
    Custom { name: String, url_template: String },
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            homepage: "cactus://newtab".into(),
            search_engine: SearchEngine::DuckDuckGo,
            restore_session: true,
            https_first: true,
            block_ads_trackers: true,
            send_dnt: true,
            send_gpc: true,
            show_bookmark_bar: true,
            default_zoom: 1.0,
        }
    }
}

impl Preferences {
    pub fn load(paths: &AppPaths) -> Result<Self> {
        let path = paths.prefs_path();
        if !path.exists() {
            let prefs = Self::default();
            prefs.save(paths)?;
            return Ok(prefs);
        }
        let text = fs::read_to_string(&path)
            .with_context(|| format!("reading prefs {}", path.display()))?;
        let prefs: Self = toml::from_str(&text).context("parsing prefs.toml")?;
        Ok(prefs)
    }

    pub fn save(&self, paths: &AppPaths) -> Result<()> {
        let path = paths.prefs_path();
        let text = toml::to_string_pretty(self).context("serializing prefs")?;
        fs::write(&path, text).with_context(|| format!("writing prefs {}", path.display()))?;
        Ok(())
    }

    pub fn search_url(&self, query: &str) -> String {
        let q = urlencoding_lite(query);
        match &self.search_engine {
            SearchEngine::DuckDuckGo => format!("https://duckduckgo.com/?q={q}"),
            SearchEngine::Startpage => format!("https://www.startpage.com/sp/search?query={q}"),
            SearchEngine::Wikipedia => {
                format!("https://en.wikipedia.org/w/index.php?search={q}")
            }
            SearchEngine::Custom { url_template, .. } => {
                url_template.replace("%s", &q).replace("{searchTerms}", &q)
            }
        }
    }
}

fn urlencoding_lite(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_duckduckgo() {
        let p = Preferences::default();
        assert!(p.search_url("hello world").contains("duckduckgo.com"));
        assert!(p.search_url("hello world").contains("hello+world"));
    }
}
