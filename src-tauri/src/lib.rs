use tauri::{Url, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

const APP_URL: &str = "https://messages.google.com/web/conversations";

const INTERNAL_HOST_SUFFIXES: &[&str] = &[
    "google.com",
    "gstatic.com",
    "googleusercontent.com",
    "googleapis.com",
    "youtube.com",
];

fn is_internal_host(host: &str) -> bool {
    INTERNAL_HOST_SUFFIXES
        .iter()
        .any(|suffix| host == *suffix || host.ends_with(&format!(".{}", suffix)))
}

// Injected on every page load. Catches clicks on external links and
// `window.open` calls and reassigns `window.location.href`. The actual routing
// to the system browser happens in Rust via `on_navigation` below — this script
// just makes sure target=_blank/window.open turn into navigations the Rust side
// can see, since IPC from remote origins is not available.
const LINK_INTERCEPTOR: &str = r#"
(() => {
  const INTERNAL = ['google.com', 'gstatic.com', 'googleusercontent.com', 'googleapis.com', 'youtube.com'];
  const isInternal = (host) => INTERNAL.some(s => host === s || host.endsWith('.' + s));

  const resolve = (raw) => {
    try { return new URL(raw, location.href); } catch { return null; }
  };

  const route = (raw) => {
    const u = resolve(raw);
    if (!u || isInternal(u.hostname)) return false;
    window.location.href = u.href;
    return true;
  };

  const onClick = (e) => {
    const a = e.target?.closest?.('a');
    if (!a || !a.href) return;
    const u = resolve(a.href);
    if (!u) return;
    if (a.target === '_blank' || !isInternal(u.hostname)) {
      e.preventDefault();
      e.stopPropagation();
      route(a.href);
    }
  };

  document.addEventListener('click', onClick, true);
  document.addEventListener('auxclick', onClick, true);

  const nativeOpen = window.open;
  window.open = function(url, target, features) {
    if (typeof url === 'string' && url && route(url)) return null;
    return nativeOpen ? nativeOpen.call(this, url, target, features) : null;
  };

  console.log('[google-messages-desktop] link interceptor installed');
})();
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let url: Url = APP_URL.parse().expect("APP_URL must be a valid URL");
            let app_handle = app.handle().clone();
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("Google Messages")
                .inner_size(1200.0, 800.0)
                .initialization_script(LINK_INTERCEPTOR)
                .on_navigation(move |target| {
                    let host = target.host_str().unwrap_or("");
                    if is_internal_host(host) {
                        return true;
                    }
                    let _ = app_handle.opener().open_url(target.as_str(), None::<&str>);
                    false
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
