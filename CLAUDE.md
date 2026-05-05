# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this project actually is

A Tauri 2 desktop wrapper around the Google Messages web client. The Tauri window loads `https://messages.google.com/web/conversations` **directly** — the window and URL are constructed in Rust at startup (`src-tauri/src/lib.rs`, see `WebviewWindowBuilder` in `run()`), not declared in `tauri.conf.json`. There is no `app.windows` entry in the config.

External link handling is split between two places that work together:

1. **`LINK_INTERCEPTOR` JS** — injected via `initialization_script`, runs on every page load. Catches `<a>` clicks (capture phase, both `click` and `auxclick`) where `target="_blank"` or the host isn't internal, and `window.open()` calls. For external URLs it does `window.location.href = url` to turn them into a real navigation. It does **not** call Tauri IPC.
2. **`on_navigation` Rust handler** — fires for every navigation the webview attempts. If the host isn't in the internal allow-list, it calls `tauri-plugin-opener`'s `open_url` to launch the system browser and returns `false` to cancel the navigation. The webview stays put.

Why this two-step dance instead of JS calling `__TAURI__.core.invoke` directly: Tauri 2 hardens IPC such that remote origins (anything loaded via `WebviewUrl::External`) cannot invoke commands without an explicit `remote.urls` entry in the capability — and even then the global `__TAURI__` isn't reliably injected on remote pages. Routing through `on_navigation` sidesteps the whole IPC question.

The internal-host allow-list (`INTERNAL_HOST_SUFFIXES` in Rust, `INTERNAL` in JS — keep them in sync) covers `google.com`, `gstatic.com`, `googleusercontent.com`, `googleapis.com`, `youtube.com` so sign-in and embedded media stay in-app.

**Known limitation:** the right-click / cmd-click context menu's "Open Link in New Window" creates a new native webview through a path Tauri 2 doesn't expose a public hook for, so those menu items don't do anything useful. Plain clicks work; that's the supported path.

The Vite/TypeScript scaffolding in `src/` and `index.html` is **not loaded by the desktop app** at runtime — the webview points at the remote URL, full stop. Those files exist only for `npm run dev` if someone wants to poke at the local shell in a browser. `npm run build` (`tsc && vite build`) still works but its output is unused. Treat `src/` as "harmless leftover scaffolding from `create-tauri-app`."

## Common commands

```bash
npm install              # install JS deps; Cargo deps fetch on first tauri build
npm run tauri dev        # run the desktop app in dev mode
npm run tauri build      # produce platform installers in src-tauri/target/release/bundle/
cargo check              # (from src-tauri/) fast Rust type-check, useful after editing lib.rs
```

There is no test suite and no linter. `tsc` (via `npm run build`) and `cargo check` are the only static checks.

## Releases & CI

`.github/workflows/build-and-release.yml` triggers on GitHub release **publish** (not push). It builds on macOS, Windows, and Ubuntu runners in parallel, then a follow-up `release` job attaches the `.dmg`, `.msi`, `.deb`, and `.AppImage` artifacts using `RELEASE_ACCESS_TOKEN`. The Linux runner needs `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf` — installed inline in the workflow.

The release workflow rewrites the version in `package.json`, `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.toml` from `github.event.release.tag_name` (stripping a leading `v`) before building, so the three source files can stay out of date — the published artifact filenames will match the release tag. If you want a local `tauri build` to produce a specific filename, you still need to edit all three files yourself; Tauri does not derive any of them from another.

## Capabilities & permissions

`src-tauri/capabilities/default.json` allows `core:default` and `opener:default`. The opener call currently happens from Rust (`on_navigation` → `OpenerExt::open_url`), so the `opener:default` permission isn't actively gating anything — it's there in case future frontend code invokes the opener plugin from JS. The capability has no `remote.urls` entry, which is intentional: nothing on `messages.google.com` is permitted to invoke Tauri commands. If you ever need that, both add `remote.urls` and verify that `__TAURI__` is actually present on the remote page (Tauri 2 has been inconsistent here — see the discussion in this file's "external link handling" section).

## File layout pointers

- `src-tauri/src/lib.rs` — Tauri builder, window creation, `LINK_INTERCEPTOR` JS. The center of gravity for app behavior.
- `src-tauri/tauri.conf.json` — bundle targets, app identifier (`net.vivin.google-messages-desktop`), `withGlobalTauri`. No window config.
- `src-tauri/capabilities/default.json` — Tauri 2 capability/permission allowlist for the main window.
- `assets/screenshot.png` — referenced by the README; don't delete.
