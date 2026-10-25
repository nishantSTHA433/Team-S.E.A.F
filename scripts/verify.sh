#!/usr/bin/env bash
# Run from Fedora with GTK development libraries and Rust installed.
set -euo pipefail
cd "$(dirname "$0")/.."
export PATH="$HOME/.cargo/bin:$PATH"
cargo fmt --check
cargo build --locked
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
# GTK must parse the generated theme, not a browser CSS validator.
gtk_test() {
    if [[ -n "${DISPLAY:-}${WAYLAND_DISPLAY:-}" ]]; then
        GSK_RENDERER=cairo G_DEBUG=fatal-criticals cargo test --locked "$1" -- --ignored --nocapture --test-threads=1
    else
        command -v xvfb-run >/dev/null || { echo "Install Xvfb or run from the Fedora desktop." >&2; exit 1; }
        xvfb-run -a env GSK_RENDERER=cairo G_DEBUG=fatal-criticals cargo test --locked "$1" -- --ignored --nocapture --test-threads=1
    fi
}
gtk_test theme::tests::gtk_stylesheet_parses
gtk_test ui::gtk_adapter_tests::gtk_focused_file_navigation_and_panel_lifecycle
if [[ -n "${LCARS_MEDIA_FIXTURE:-}" ]]; then
    [[ -f "$LCARS_MEDIA_FIXTURE" ]] || { echo "Media fixture not found" >&2; exit 1; }
    gtk_test ui::gtk_adapter_tests::gtk_adapters_preserve_input_and_shared_video_stream
else
    echo "Media/navigation integration NOT RUN: set LCARS_MEDIA_FIXTURE to a seekable test video."
fi
git diff --check
git fsck --full
echo "Requested verification checks passed."
