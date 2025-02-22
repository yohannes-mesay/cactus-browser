use crate::lists::FilterListManager;
use adblock::Engine;
use anyhow::Result;
use cactus_core::AppPaths;
use log::info;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BlockDecision {
    pub matched: bool,
    pub important: bool,
    pub redirect: Option<String>,
    pub rewritten_url: Option<String>,
}

/// Adblock engine wrapper.
///
/// NOTE: `adblock::Engine` is intentionally `!Send`/`!Sync`, so this lives on the
/// GTK main thread behind `Rc<RefCell<_>>`.
pub struct PrivacyEngine {
    engine: RefCell<Option<Engine>>,
    enabled: RefCell<bool>,
}

impl PrivacyEngine {
    pub fn new(enabled: bool) -> Rc<Self> {
        Rc::new(Self {
            engine: RefCell::new(None),
            enabled: RefCell::new(enabled),
        })
    }

    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.borrow_mut() = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        *self.enabled.borrow()
    }

    /// Build the engine on the calling thread (must be the GTK main thread).
    pub fn load(&self, paths: &AppPaths) -> Result<()> {
        let manager = FilterListManager::new(paths);
        let lists = manager.ensure_lists()?;
        let mut filter_set = adblock::lists::FilterSet::new(true);
        for list in &lists {
            filter_set.add_filters(list.lines(), adblock::lists::ParseOptions::default());
        }
        let engine = Engine::from_filter_set(filter_set, true);
        info!(
            "privacy engine ready ({} filter list files, enabled={})",
            lists.len(),
            self.is_enabled()
        );
        *self.engine.borrow_mut() = Some(engine);
        Ok(())
    }

    pub fn should_block(&self, url: &str, source_url: &str, request_type: &str) -> BlockDecision {
        if !self.is_enabled() {
            return BlockDecision {
                matched: false,
                important: false,
                redirect: None,
                rewritten_url: None,
            };
        }
        let guard = self.engine.borrow();
        let Some(engine) = guard.as_ref() else {
            return BlockDecision {
                matched: false,
                important: false,
                redirect: None,
                rewritten_url: None,
            };
        };

        let req = match adblock::request::Request::new(url, source_url, request_type) {
            Ok(r) => r,
            Err(_) => {
                return BlockDecision {
                    matched: false,
                    important: false,
                    redirect: None,
                    rewritten_url: None,
                }
            }
        };
        let result = engine.check_network_request(&req);
        BlockDecision {
            matched: result.matched,
            important: result.important,
            redirect: result.redirect,
            rewritten_url: result.rewritten_url,
        }
    }

    pub fn cosmetic_styles(&self, url: &str) -> String {
        if !self.is_enabled() {
            return String::new();
        }
        let guard = self.engine.borrow();
        let Some(engine) = guard.as_ref() else {
            return String::new();
        };
        let result = engine.url_cosmetic_resources(url);
        let mut css = String::new();
        if !result.hide_selectors.is_empty() {
            let selectors: Vec<String> = result.hide_selectors.into_iter().collect();
            css.push_str(&selectors.join(", "));
            css.push_str(" { display: none !important; }\n");
        }
        css
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_known_tracker_when_loaded() {
        let paths = AppPaths {
            config_dir: std::env::temp_dir().join("cactus-priv-cfg2"),
            data_dir: std::env::temp_dir().join("cactus-priv-data2"),
            cache_dir: std::env::temp_dir().join("cactus-priv-cache2"),
        };
        let _ = std::fs::remove_dir_all(&paths.cache_dir);
        paths.ensure().unwrap();
        std::fs::create_dir_all(paths.filter_cache_dir()).unwrap();
        std::fs::write(
            paths.filter_cache_dir().join("list-0.txt"),
            "||doubleclick.net^\n",
        )
        .unwrap();
        std::fs::write(paths.filter_cache_dir().join("list-1.txt"), "").unwrap();

        let engine = PrivacyEngine::new(true);
        engine.load(&paths).unwrap();
        let decision = engine.should_block(
            "https://doubleclick.net/ads.js",
            "https://example.com/",
            "script",
        );
        assert!(decision.matched);
    }
}
