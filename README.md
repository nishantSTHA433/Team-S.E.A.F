# LCARS Console

A Rust/GTK4 Fedora desktop console styled after the supplied LCARS reference.

Working source inside Fedora: `/home/fedora/RustroverProjects/lcars-desktop`.
Mac source export: `/Users/Vontrell/Projects/LCARS-Exchange/lcars-v2`.
The exchange is mounted at `/mnt/macos` in Fedora. Exports are not automatically synchronized with RustRover edits.

Open the Fedora project in RustRover. From its terminal:

```sh
cargo run
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

The Fedora application menu also provides **LCARS Console**. F11 toggles fullscreen;
Escape leaves fullscreen. Drag the title strip to move the window.

[Current features, validation, graphics mitigation and limitations](docs/update-2026-10-02.md).

Source layout: `src/ui.rs` (console and file manager), `src/network.rs` (live network events and speed test), `src/tools.rs` (calculator and diagnostics), `src/services.rs` (telemetry/preferences/drive labels), `assets/lcars.css` (visual design).

Rust is pinned to 1.98.1 and dependencies are recorded in Cargo.lock. Native GTK4 development packages are required. GTK 4.8 API features work with this VM's GTK 4.10.5. Fedora 38 is end-of-life; migrate before production deployment.

Use Cargo directly for continued development. Exchange installation scripts are setup aids and copy sources; do not rerun them over subsequent project edits.
