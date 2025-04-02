# Cactus

**Cactus** is a native, Linux-first web browser written in Rust.

It wraps **WebKitGTK 6** with a **GTK4 + libadwaita** chrome — Wayland-first, XDG-correct, privacy-on by default, and not another Chromium skin.

## Why Cactus

Linux users are stuck between heavy Chromium clones and browsers that feel poorly integrated with the desktop. Cactus aims to be a daily driver that:

- Feels native on GNOME (and acceptable elsewhere via libadwaita)
- Blocks ads & trackers by default (EasyList + EasyPrivacy via Brave’s `adblock` engine)
- Uses HTTPS-First, DNT, and Global Privacy Control
- Stores data under XDG paths (`~/.config/cactus`, `~/.local/share/cactus`)
- Ships **zero first-party telemetry**

## Features (v0.1)

- Multi-window / multi-tab browsing (`AdwTabView`)
- Omnibox (URL + search), find-in-page, zoom
- Bookmarks bar, history, downloads, session restore
- Private windows (ephemeral WebKit network session)
- Internal pages: `cactus://newtab`, `settings`, `history`, `bookmarks`, `downloads`, `about`
- Keyboard shortcuts: `Ctrl+T/W/L/R`, `Ctrl+Tab`, `Alt+←/→`, `Ctrl+Shift+N`, `Ctrl+D/H/J/F`

## Requirements

System packages (Ubuntu/Debian example):

```bash
sudo apt install \
  build-essential pkg-config \
  libgtk-4-dev libadwaita-1-dev libwebkitgtk-6.0-dev \
  libsqlite3-dev
```

Fedora:

```bash
sudo dnf install gtk4-devel libadwaita-devel webkitgtk6.0-devel sqlite-devel
```

Arch:

```bash
sudo pacman -S gtk4 libadwaita webkitgtk-6.0 sqlite
```

Rust: stable toolchain via [rustup](https://rustup.rs/).

## Build & run

If system `-dev` packages are installed:

```bash
cargo build --release
./target/release/cactus
```

On this repo’s machine (or when headers live under `.deps/`), use the helper env:

```bash
source scripts/env.sh
cargo run
```

Optional private window:

```bash
cargo run -- --private
```

## Install (local)

```bash
source scripts/env.sh
cargo build --release
sudo install -Dm755 target/release/cactus /usr/local/bin/cactus
sudo install -Dm644 data/org.cactus.Browser.desktop \
  /usr/local/share/applications/org.cactus.Browser.desktop
sudo install -Dm644 data/org.cactus.Browser.metainfo.xml \
  /usr/local/share/metainfo/org.cactus.Browser.metainfo.xml
sudo install -Dm644 data/icons/org.cactus.Browser.svg \
  /usr/local/share/icons/hicolor/scalable/apps/org.cactus.Browser.svg
update-desktop-database ~/.local/share/applications 2>/dev/null || true
```

A Flatpak manifest lives at [`flatpak/org.cactus.Browser.yml`](flatpak/org.cactus.Browser.yml).

## Configuration

Preferences: `~/.config/cactus/prefs.toml`

```toml
homepage = "cactus://newtab"
restore_session = true
https_first = true
block_ads_trackers = true
send_dnt = true
send_gpc = true
show_bookmark_bar = true
default_zoom = 1.0

[search_engine]
type = "duck_duck_go"
```

SQLite DB: `~/.local/share/cactus/cactus.db`  
Filter cache: `~/.cache/cactus/filters/`

## Architecture

```
crates/
  cactus/           # binary entrypoint
  cactus-ui/        # Adw windows, tabs, omnibox, shortcuts
  cactus-webkit/    # WebView + NetworkSession helpers
  cactus-privacy/   # adblock engine + filter lists
  cactus-core/      # prefs, SQLite store, URL/helpers, internal HTML
```

## Tests

```bash
source scripts/env.sh
cargo test -p cactus-core -p cactus-privacy
```

Manual checklist: [`MANUAL_TEST.md`](MANUAL_TEST.md).

## Non-goals (v0.1)

- Custom browser engine / Servo
- Chrome extension compatibility
- Sync accounts
- Password manager / payment autofill (deferred — high security surface)
- Non-Linux ports

## License

MPL-2.0
