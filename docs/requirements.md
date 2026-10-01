# Requirement and acceptance checklist

Status describes implementation scope, not client acceptance or physical-hardware certification.

| BR | Requirement | Prototype / next acceptance |
|---|---|---|
| 01–02 | Approved LCARS aesthetic; exclude DS9/Kelvin | Original LCARS-inspired shapes and colors implemented; client reference-board approval pending |
| 03 | Touch operation | Large controls, single-selection details, explicit actions; real touch and on-screen keyboard testing pending |
| 04 | High visual quality | Reusable CSS and five pages; visual review required |
| 05 | Useful live hardware information | Guest CPU/memory/uptime/disk/network implemented; physical sensors unavailable in VM |
| 06 | Discover native applications | GIO visible-app enumeration, filtering and launch implemented |
| 07 | Persistent network indicator | Footer indicator distinguishes online/local/limited/portal/offline/unknown |
| 08 | Offline visual and sound alerts | Persistent visual state, optional online-to-offline sound; event subscriptions/audio validation pending |
| 09 | Diagnose and resolve network issues | Read-only local diagnostics and Fedora settings launcher; repair workflows pending |
| 10 | Speed test | Explicitly unconfigured pending provider and data budget; not implemented |
| 11 | Mounted drives with C:/D:/E: labels | C: links to Linux root, mounted-volume information; persistent additional mappings pending |
| 12 | Selection metadata | Type, size, permissions, path and raw modified timestamp implemented; richer formatting pending |
| 13 | Open/Delete | Default app or folder open, confirmation and recoverable Trash; no permanent-delete fallback |
| 14 | Custom graphics/icons | Styled interface and simple geometric file/folder glyphs; complete icon family pending |

## Manual acceptance sequence

1. Open application without sudo; inspect all five pages.
2. Resize window and toggle fullscreen with F11; leave it with Escape.
3. Check live CPU and uptime values update while navigating.
4. Search visible applications and launch a harmless utility.
5. Browse a temporary directory, inspect metadata and open a text file.
6. Move only a disposable test file to Trash; verify it can be recovered using Fedora Files.
7. Run local diagnostics and verify UI remains responsive.
8. Test network state transitions only in a disposable VM/network session.
9. Reopen to verify saved preferences.
10. Repeat on physical touch hardware and target architecture before release.

## Architecture boundaries

UI widgets stay on the GTK main thread. Telemetry and local diagnostics run in
workers. Directory enumeration is background work and trash uses GIO async I/O.
The app does not replace GNOME, embed external application windows, alter network
configuration, format drives, bypass Linux file permissions, or run as root.

## Known limitations to address before broader use

- Initial folder listing is capped at 2,000 entries; pagination/search required.
- State sampling is not instant network loss detection; define target latency.
- No full speed test, drive-letter persistence, packaging, favorites or compositor.
- Fedora 38 has no ongoing security support.
- UTM does not expose every host sensor and is not proof of touch compatibility.
