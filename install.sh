#!/usr/bin/env bash
# Claude AI continuation (user-attributed): Step 18 release installer.
# See docs/WORK_PROVENANCE.md; attribution is separate from final acceptance.
# Repeatable install for the LCARS console. Builds an optimised binary, installs it
# into the user prefix and registers a desktop entry. Re-running it updates in place.
set -euo pipefail
src=$(cd "$(dirname "$0")" && pwd)
prefix=${PREFIX:-$HOME/.local}
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release --manifest-path "$src/Cargo.toml"
install -D -m 755 "$src/target/release/lcars-desktop" "$prefix/bin/lcars-desktop"
install -d "$prefix/share/applications"
cat > "$prefix/share/applications/org.lcars.Console.desktop" <<DESKTOP
[Desktop Entry]
Type=Application
Name=LCARS Console
Comment=Fedora workstation console
Exec=env GSK_RENDERER=cairo $prefix/bin/lcars-desktop
Icon=utilities-system-monitor
Terminal=false
Categories=Utility;System;
DESKTOP
update-desktop-database "$prefix/share/applications" 2>/dev/null || true
echo "INSTALLED $prefix/bin/lcars-desktop"
echo "To start on login: cp $prefix/share/applications/org.lcars.Console.desktop ~/.config/autostart/"
