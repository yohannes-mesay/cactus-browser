# Cactus v0.1 manual test checklist

## Build & launch
- [ ] `source scripts/env.sh && cargo build`
- [ ] `cargo run` opens a window on Wayland (or X11 fallback)
- [ ] App id / window class is `org.cactus.Browser`

## Navigation chrome
- [ ] New tab (`Ctrl+T`) opens `cactus://newtab`
- [ ] Omnibox loads `example.com` as HTTPS
- [ ] Omnibox search query goes to DuckDuckGo
- [ ] Back / forward / reload / home work
- [ ] `Ctrl+L` focuses the URL bar
- [ ] `Ctrl+Tab` / `Ctrl+Shift+Tab` cycle tabs
- [ ] Zoom in/out/reset (`Ctrl+=`, `Ctrl+-`, `Ctrl+0`)
- [ ] Find in page (`Ctrl+F`)

## Persistence
- [ ] Bookmark current page (`Ctrl+D`) appears on bookmark bar
- [ ] History page (`Ctrl+H` / `cactus://history`) lists visits
- [ ] Downloads appear under `cactus://downloads`
- [ ] Quit and relaunch restores previous tabs

## Privacy
- [ ] Private window (`Ctrl+Shift+N`) shows PRIVATE badge
- [ ] Private browsing does not write history/bookmarks
- [ ] Toggle ad blocking from menu; EasyList/EasyPrivacy load into cache
- [ ] Clear data removes cookies/cache/history
- [ ] `cactus://settings` reflects HTTPS-First / DNT / GPC defaults
- [ ] No outbound telemetry from Cactus itself

## Desktop integration
- [ ] `.desktop` file installs and appears in app launcher
- [ ] Opening an `https://` link launches Cactus when set as handler
- [ ] Dark/light follows system via libadwaita

## Known v0.1 gaps
- Password manager / autofill not implemented
- Chrome extension compatibility not supported
- Network request cancellation for blocked resources is best-effort (cosmetic filtering always applied)
