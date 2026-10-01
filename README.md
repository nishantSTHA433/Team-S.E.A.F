# LCARS Console — Fedora development prototype

A Rust/GTK4 workstation interface. GNOME remains the desktop/session manager.

## Locations

- Working source inside Fedora: `/home/fedora/RustroverProjects/lcars-desktop`
- Mac exchange: `/Users/Vontrell/Projects/LCARS-Exchange`
- The same exchange inside Fedora: `/mnt/macos`
- Existing installation test preserved: `/home/fedora/RustroverProjects/rustrover_check`
- UTM rollback copy: `Fedora 38 — Before LCARS 2026-10-01`

The Mac exchange contains the source delivery and logs, not a live mirror of edits
made in RustRover. Keep active builds on Fedora's filesystem. Export changes
deliberately or use a Git remote for an independent backup. The UTM copy is on the
same Mac disk and does not protect against loss of that disk.

## Open and run

In RustRover, choose Open and select the working source directory. Open Cargo.toml
as a project if prompted. Use a Cargo run configuration with command `run` and the
project root as working directory. The Fedora application menu also contains
`LCARS Console` after successful setup.

```bash
cd /home/fedora/RustroverProjects/lcars-desktop
cargo run
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

Rust is pinned to 1.98.1. GTK bindings use the 0.9 release family with GTK 4.8 API
features, compatible with the observed GTK 4.10.5 on this VM. Cargo.lock records
the resolved dependencies. Fedora 38 is end-of-life; it is a compatibility
prototype baseline, not the recommended production OS.

## Interface

- Overview: real guest CPU, memory, uptime, network rates and home filesystem use.
- Files: folder browsing, metadata, default-app opening, explicit Trash dialog.
  There is no permanent-delete fallback. Enumeration is limited to 2,000 entries
  per folder in this first version. File operations honor Fedora permissions.
- Network: NetworkManager state, local adapters/routes/DNS report, shortcut to
  Fedora network settings. No automatic repair or external probes are run.
- Applications: installed visible desktop applications, searchable and launchable
  through GIO.
- Preferences: optional transition alert, persisted in
  `~/.config/lcars-desktop/preferences.json`. Sound defaults off.
- F11: fullscreen; Escape: leave fullscreen; Exit: close application.

## Project layout

- `src/main.rs`: application entry point.
- `src/ui.rs`: GTK pages, reusable controls and interaction wiring.
- `src/services.rs`: background measurements, NetworkManager parsing, local
  diagnostics, preferences and behavior tests.
- `assets/lcars.css`: LCARS colors, geometry, control states and type scale.
- `docs/requirements.md`: acceptance status and next implementation priorities.

## Data semantics and constraints

Measurements are for Fedora in UTM, not the physical Mac. Network traffic is the
sum of non-loopback guest interfaces and can double-count tunneled traffic.
Traffic rate is not an internet speed test. Network status is sampled every five
seconds; internet verification follows NetworkManager's configured policy.
The transition alert is best effort and uses `canberra-gtk-play` if available.

The UI does not run as root. Setup uses sudo only for OS dependencies and file
installation, then Cargo runs as `fedora`. No credentials are stored in source
or scripts. No background automation or SSH server is installed.

## Next work

1. Review the visual baseline with the client and adjust approved era references.
2. Move to a supported Fedora version when Mac storage permits, preserving this VM.
3. Test actual touch hardware, scaling, keyboard focus and the on-screen keyboard.
4. Add persistent volume identity/drive-letter mappings and richer file timestamps.
5. Add NetworkManager event subscriptions, validated sound and external diagnostic
   probes with clear destinations and timeouts.
6. Select a speed-test provider and transfer limits, then implement progress and
   cancellation. The current interface explicitly states it is unconfigured.
7. Add packaging and a separate x86-64 build/test if the target PC uses that CPU.
8. Plan dedicated desktop-session integration separately from this application.

Setup scripts in the exchange are one-time installation aids. Do not rerun the
source-copy build script after editing the Fedora project; use Cargo directly.
