# Google Messages Desktop

A simple, lightweight, unofficial desktop wrapper for Google Messages, built with [Tauri](https://tauri.app/).

![Screenshot of the Google Messages Desktop app](./assets/screenshot.png)

## Features

* **Lightweight**: Uses your operating system's native WebView, resulting in a tiny app size and low memory usage.
* **Cross-Platform**: Can be built for macOS, Windows, and Linux from a single codebase.
* **Simple**: No extra frills. Just the Google Messages web client in its own dedicated window.

## Installation

You can download the latest version for your operating system from the [Releases](https://github.com/vivin/google-messages-desktop/releases) page.

* **macOS**: Download the `.dmg` file. Open it and drag the app to your Applications folder. **First-time launch:** see the macOS section below — the app isn't codesigned, so macOS will refuse to open it the first time with a misleading "damaged" message.
* **Windows**: Download the `.msi` file and run the installer. SmartScreen may show an "unrecognized app" warning the first time; click "More info" → "Run anyway."
* **Linux**: For Debian, Ubuntu, or derived distros use the `.deb` file. Otherwise use the `.AppImage` file (which will work on any Linux distro).

### macOS: "App is damaged" on first launch

The app isn't signed with an Apple Developer ID (that requires a paid Apple account), so on first launch macOS will say *"Google Messages Desktop is damaged and can't be opened."* The app isn't actually damaged — that's macOS's stock message for any app it doesn't recognize. You only need to do this once; after the first successful open, macOS remembers it.

**Option 1 — System Settings (no terminal):**
1. Drag the app into your Applications folder.
2. Double-click it. You'll get the "damaged" warning. Click **Cancel**.
3. Open **System Settings → Privacy & Security**. Scroll down to the *Security* section.
4. You'll see a line about *"Google Messages Desktop was blocked from use because it is not from an identified developer."* Click **Open Anyway**.
5. Authenticate, then click **Open** in the confirmation dialog. The app launches.

**Option 2 — Terminal (one command):**
```bash
xattr -dr com.apple.quarantine "/Applications/Google Messages Desktop.app"
```
After running it, double-click the app normally.

## Building from Source

If you'd like to build the application yourself, you'll need to set up the Tauri development environment.

### Prerequisites

* [Rust and Cargo](https://www.rust-lang.org/tools/install)
* [Node.js and npm](https://nodejs.org/en/)
* [Tauri CLI Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) for your specific OS.

### Steps

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/vivin/google-messages-desktop.git
    cd google-messages-desktop
    ```

2.  **Install NPM dependencies:**
    ```bash
    npm install
    ```

3.  **Run in development mode:**
    ```bash
    npm run tauri dev
    ```

4.  **Build the application:**
    ```bash
    npm run tauri build
    ```
    The final executables will be located in `src-tauri/target/release/bundle/`.

## Disclaimer

This project is not affiliated with, endorsed by, or sponsored by Google in any way. It is an unofficial, third-party application that provides a wrapper around the official Google Messages web client. All trademarks and logos are the property of their respective owners.

## License

This project is licensed under the MIT License.
