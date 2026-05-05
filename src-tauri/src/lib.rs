use tauri::{Url, WebviewUrl, WebviewWindowBuilder};

const APP_URL: &str = "https://messages.google.com/web/conversations";

// Injected on every page load. Routes external link clicks and window.open()
// calls through tauri-plugin-opener so they launch in the system browser
// instead of replacing the app's webview or being silently dropped.
const LINK_INTERCEPTOR: &str = r#"
(() => {
  const INTERNAL_SUFFIXES = [
    'google.com',
    'gstatic.com',
    'googleusercontent.com',
    'googleapis.com',
    'youtube.com'
  ];

  const isInternal = (raw) => {
    try {
      const host = new URL(raw, location.href).hostname;
      return INTERNAL_SUFFIXES.some(s => host === s || host.endsWith('.' + s));
    } catch {
      return true;
    }
  };

  const openExternally = (url) => {
    window.__TAURI__?.core?.invoke?.('plugin:opener|open_url', { url });
  };

  document.addEventListener('click', (e) => {
    const a = e.target?.closest?.('a');
    if (!a || !a.href) return;
    if (a.target === '_blank' || !isInternal(a.href)) {
      e.preventDefault();
      e.stopPropagation();
      openExternally(a.href);
    }
  }, true);

  const nativeOpen = window.open;
  window.open = function(url, target, features) {
    if (typeof url === 'string' && url) {
      openExternally(url);
      return null;
    }
    return nativeOpen?.call(this, url, target, features) ?? null;
  };
})();
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let url: Url = APP_URL.parse().expect("APP_URL must be a valid URL");
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("Google Messages")
                .inner_size(1200.0, 800.0)
                .initialization_script(LINK_INTERCEPTOR)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
