use anyhow::{Context, Result};
use cactus_core::AppPaths;
use log::{info, warn};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub const DEFAULT_FILTER_URLS: &[&str] = &[
    "https://easylist.to/easylist/easylist.txt",
    "https://easylist.to/easylist/easyprivacy.txt",
];

const MAX_AGE: Duration = Duration::from_secs(60 * 60 * 24 * 7);

pub struct FilterListManager {
    cache_dir: PathBuf,
}

impl FilterListManager {
    pub fn new(paths: &AppPaths) -> Self {
        Self {
            cache_dir: paths.filter_cache_dir(),
        }
    }

    pub fn ensure_lists(&self) -> Result<Vec<String>> {
        fs::create_dir_all(&self.cache_dir)?;
        let mut rules = Vec::new();
        for (idx, url) in DEFAULT_FILTER_URLS.iter().enumerate() {
            let path = self.cache_dir.join(format!("list-{idx}.txt"));
            if needs_refresh(&path) {
                match download(url, &path) {
                    Ok(()) => info!("updated filter list from {url}"),
                    Err(err) => {
                        warn!("failed to refresh {url}: {err}");
                        if !path.exists() {
                            // Seed a minimal offline set so the engine still works.
                            fs::write(&path, MINIMAL_FILTERS)?;
                        }
                    }
                }
            }
            let text = fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            rules.push(text);
        }
        Ok(rules)
    }
}

fn needs_refresh(path: &PathBuf) -> bool {
    match fs::metadata(path).and_then(|m| m.modified()) {
        Ok(modified) => SystemTime::now()
            .duration_since(modified)
            .map(|d| d > MAX_AGE)
            .unwrap_or(true),
        Err(_) => true,
    }
}

fn download(url: &str, dest: &PathBuf) -> Result<()> {
    let response = ureq::get(url)
        .set("User-Agent", "Cactus/0.1 (+https://cactus.browser)")
        .call()
        .with_context(|| format!("GET {url}"))?;
    let text = response.into_string().context("reading filter body")?;
    let tmp = dest.with_extension("tmp");
    fs::write(&tmp, &text)?;
    fs::rename(tmp, dest)?;
    Ok(())
}

const MINIMAL_FILTERS: &str = r#"
! Cactus offline bootstrap filters
||doubleclick.net^
||googlesyndication.com^
||google-analytics.com^
||googletagmanager.com^
||facebook.net^
||scorecardresearch.com^
||adservice.google.com^
||pagead2.googlesyndication.com^
"#;
