use crate::window::{build_window, restore_urls, AppState};
use anyhow::Result;
use cactus_core::{AppPaths, Preferences, Store, APP_ID, NEW_TAB_URI};
use cactus_privacy::PrivacyEngine;
use cactus_webkit::SessionBundle;
use gtk4::prelude::*;
use log::{info, warn};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

pub fn run(private: bool) -> Result<()> {
    let paths = AppPaths::discover()?;
    let prefs = Preferences::load(&paths)?;
    let store = Arc::new(Store::open(&paths)?);
    let privacy = PrivacyEngine::new(prefs.block_ads_trackers);

    // Engine is !Send; load on the main thread. Cached lists make this fast after first run.
    if let Err(err) = privacy.load(&paths) {
        warn!("privacy engine load failed: {err}");
    }

    let normal_session = SessionBundle::persistent(&paths)?;

    let state = Rc::new(AppState {
        paths,
        prefs: RefCell::new(prefs),
        store,
        privacy,
        normal_session,
    });

    let app = libadwaita::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(move |app| {
        if let Some(win) = app.active_window() {
            win.present();
            return;
        }

        let urls = if private {
            vec![NEW_TAB_URI.into()]
        } else {
            restore_urls(&state).unwrap_or_else(|| vec![NEW_TAB_URI.into()])
        };
        info!(
            "starting {} window with {} tab(s)",
            if private { "private" } else { "normal" },
            urls.len()
        );
        let window = build_window(app, state.clone(), private, Some(urls));
        window.present();
    });

    let status = app.run();
    if status != glib::ExitCode::SUCCESS {
        anyhow::bail!("application exited with {status:?}");
    }
    Ok(())
}
