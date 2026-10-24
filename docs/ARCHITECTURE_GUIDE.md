# LCARS project architecture — inspected 2026-10-22

## Start here

The authoritative project is `/home/fedora/RustroverProjects/lcars-desktop` inside Fedora. Open that folder in RustRover. `src/` contains Rust logic, `assets/lcars.css` contains visual styling, `Cargo.toml` defines the Rust package/dependencies, and `WORK_TRACKER.md` is the execution record. `docs/WORK_PROVENANCE.md` records the Codex/Claude boundary.

This is one native Rust program using GTK4 for windows, controls, events and media widgets, with Cairo drawing for custom LCARS instruments. It is not a web page or a replacement Fedora operating system. The six built-in systems are panels within that one process. Other Fedora programs launched from Applications are separate processes with their own windows.

## Why Fedora sees an application in Activities

1. `install.sh` runs a release build and copies it to `/home/fedora/.local/bin/lcars-desktop` by default.
2. It creates `/home/fedora/.local/share/applications/org.lcars.Console.desktop`. That desktop entry tells GNOME the display name (LCARS Console), icon, categories and launch command. The inspected installed command is `env GSK_RENDERER=cairo /home/fedora/.local/bin/lcars-desktop`, with `Terminal=false`.
3. `src/main.rs::main()` creates a `gtk::Application` with the identifier `org.lcars.Console`, connects activation to `ui::build`, then enters GTK's event loop with `app.run()`. The matching application identity supplies the association used for the app and its notifications. Fedora's Activities view is showing a registered application and its window, not an internal LCARS panel.
4. `ui::build()` creates and presents the main ApplicationWindow. GTK dispatches touches/clicks, key events, redraws and timers for its lifetime.

RustRover edits/builds the source; it does not run the installed copy automatically when a source file changes. `launch.sh` is the older development route and starts `target/debug/lcars-desktop`. `install.sh` rebuilds and updates the installed release copy. This distinction matters when an edit appears absent from the Activities-launched app. Optional autostart is offered by the installer; this inspection did not enable it. Exit leaves Fedora running.

## Source map

All paths below are relative to the Fedora project root.

| File | Responsibility and useful entry points |
|---|---|
| `src/main.rs` | Startup, `org.lcars.Console`, Cairo renderer default, optional `--speed-check` command-line path |
| `src/ui.rs` | `build`, `start_menu`, `settings_page`, `files_page`, `load_directory`, `media_preview`, `applications_page`, `network_page`, `render_network_findings`; shell and wiring between pages/services |
| `src/console.rs` | `Surface` runtime controller; `choose`, `home`, `maximize`, `minimize`, `pack`, `layout`, `animate`, `update_attention`, `toggle_arrange`, `cancel_arrange`; panel position, focus, visibility and motion |
| `src/layout.rs` | `solve`, `Request`, `Solution`: calculate panel rectangles from available space/content requirements |
| `src/workspace.rs` | Panel identities, content contracts, workspace state/events; toolkit-independent model used by the layout layer. Live GTK state also remains in console/ui, so this is not the sole runtime state owner |
| `src/attention.rs` | `Demand`, task priorities and `Contacts`: temporary space emphasis and tracking active input |
| `src/overview.rs` | Cairo home-summary drawing: `draw_system`, `draw_network`, `draw_table`, `draw_tools`; readable summaries rather than screenshots of shrunken pages |
| `src/theme.rs` | Shared colours, `FONT` (currently Oswald), `RAIL` (10), per-panel `ACCENTS`, generated `stylesheet()` |
| `assets/lcars.css` | GTK visual rules: rails, elbows, typography, controls, lists, menus, dialogs and interaction states |
| `src/network.rs` | `subscribe` for NetworkManager events, `classify` for user-facing state, `speed_test` for bounded measurements |
| `src/services.rs` | `monitor`, `Snapshot`, `Preferences`, `command`, `diagnostics`, structured network/storage/process parsers and drive aliases |
| `src/tools.rs` | `page` for calculator/system scans, `render_findings` for structured diagnostic results, `speed_panel` for start/cancel/progress/result controls |
| `src/lib.rs` | Exposes toolkit-independent layout/workspace/attention modules to the application and tests |
| `install.sh`, `launch.sh` | Installed release application versus source-tree debug launch |

## How the UI moves and adapts

`ui::build()` makes the six systems and the compact header, Start menu and taskbar. Navigation reaches `console.rs`, whose `Surface` keeps track of focused, hidden and arranged panels. Choosing a system focuses it; choosing its active title again returns home. Minimize hides it and repacks the remaining panels; Start/taskbar can reopen it. Maximize focuses one system inside LCARS, which is separate from making the entire Fedora application window fullscreen.

At home, `overview.rs` draws compact summaries from live source data. Focused systems use real GTK controls. Existing widgets are retained/reparented rather than rebuilding all application state on every switch. `adaptive_row()` changes row/column composition as space changes. File selection, entries and long-running work therefore have a chance to survive navigation instead of being reset by it. Video explicitly pauses when its widget unmaps; retained state does not mean playback always continues in the background.

`Surface::pack()` asks `layout::solve()` for rectangles using viewport dimensions, visible systems, content contracts and current task demand. Media preview, speed tests and diagnostics can request extra emphasis through `attention::Demand`. `logo_region()` and `emblem_target()` select exposed space for the identity; `draw_emblem()` draws it. These are deterministic rules, not a trained AI or a service making decisions online.

`animate()` captures the displayed starting geometry, and `layout()` moves panels and emblem toward their targets. Reduced motion skips interpolation. Input/contact guards defer movement during active interaction, and moving/hidden content is prevented from acting like an invisible button. Basic Arrange swaps positions; Advanced permits movement/resizing. Entering Arrange snapshots the layout; Cancel restores it, while Done saves. Normal operation locks manual manipulation.

There are still important limits: the solver's overflow canvas is normalized into the visible viewport by the current console code, which keeps panels visible but can reduce space below their preferred readable dimensions at small sizes. External applications are not embedded or reflowed inside these panels. The tracker retains final touch, pointer and visual acceptance work.

## Online/offline and notifications: exact flow

`Fedora NetworkManager → network::subscribe() → classify() → callback in ui::build() → status/banner + notification + optional sound + event log`

`subscribe()` asynchronously attaches a GIO D-Bus proxy to NetworkManager on the system bus. It reads the cached `State` and `Connectivity` properties immediately, listens for property changes and service-owner changes, and retains the proxy for the application's lifetime. There is a 60-second timer retaining that proxy; it is not a 60-second internet ping loop.

Classification is explicit: unknown NM state yields UNKNOWN; asleep/disconnected/disconnecting state values yield OFFLINE even if an old connectivity property still says full; connectivity full yields ONLINE; portal yields CAPTIVE PORTAL; limited yields LIMITED; remaining cases yield LOCAL / UNVERIFIED. These labels report NetworkManager's knowledge. They do not guarantee every website is reachable, and a Wi-Fi/Ethernet link alone is not proof of internet access.

The callback in `ui.rs` immediately updates the taskbar's NETWORK text and Network page heading. Every state other than ONLINE shows the COMMUNICATIONS ALERT banner and warning styling, including UNKNOWN. It compares with `last_state`: repeated identical reports do not emit another notification. Startup ONLINE is quiet; startup degraded does notify. Later state changes notify, so repeated changes between different degraded states can each produce a new event.

Notifications use `gio::Notification` and `Application::send_notification` with the stable ID `lcars-network`, allowing the previous notification to be replaced. Degraded notifications are urgent; recovery is normal. The current title says connection unavailable for any degraded state, with the more precise state in the body. GNOME notification settings determine how system popups are presented. If the sound preference is on, a worker runs `canberra-gtk-play -i dialog-warning`; that same sound is used on recovery as well. Audio command failure is currently ignored.

Each changed state, including the initial one, is appended with local timestamp to `/home/fedora/.local/state/lcars-desktop/network-events.log`. Log write failures are ignored. This monitor lives inside the LCARS process: closing LCARS stops its monitoring and its own notifications. NetworkManager itself continues running as Fedora's service.

A second path supplies telemetry: `services::monitor()` samples `/proc` about once per second, runs `nmcli` roughly every five samples and `df` roughly every fifteen, then sends `Snapshot` data through a channel. A 250 ms GTK timer drains it, updates instruments/graphs and flags stale data. Thus overview telemetry can lag the event-driven status/banner; there are currently two sources of network state, not one universal store.

## Speed tests and diagnostics

`tools::speed_panel()` starts `network::speed_test()` on a worker and receives phase/results over a channel. Cancel sets an atomic flag checked between phases and while the child runs. The test uses curl against Cloudflare: an HTTP-response timing request, a 10 MB download and a 2 MB generated upload. It does not upload a user document. Each curl has connection/overall timeouts and the worker can kill it. This is a bounded single-connection sample, not a claim of maximum line capacity; the displayed HTTP response time is not a pure ICMP ping measurement.

System diagnostics collect kernel, storage and process information, parse rows and render headings/tables/bars through `tools::render_findings()`. Network diagnostics collect NetworkManager status, adapters, addresses, routes and DNS; `ui::render_network_findings()` displays the structured sections. The basic report is collected in a worker, but the current network rendering callback also invokes additional commands on the GTK thread, so not every diagnostic command is asynchronous. These tools inspect; they do not automatically repair a connection or terminate processes. FEDORA NETWORK SETTINGS opens the normal separate settings application.

## Files, media and native applications

`load_directory()` gathers entries and metadata, while `files_page()` wires paths, filtering, navigation, selection, Open and recoverable Trash. `media_preview()` uses GTK Picture/Video widgets, associated properties and an expanded viewer; generation checks help avoid displaying an old asynchronous result after selection changes. Drive aliases are labels over real Fedora paths, not Windows partitions.

`applications_page()` and Start discover applications through GIO AppInfo, filter names and launch their registered commands. LCARS records launch feedback and provides RETURN TO CONSOLE. A launched program remains its own Fedora window; the LCARS solver cannot rearrange its internal controls or restyle it.

## Saved data and limits of this review

Preferences default to `/home/fedora/.config/lcars-desktop/preferences.json` (respecting XDG_CONFIG_HOME). Spatial geometry/visibility use `/home/fedora/.config/lcars-desktop/console-layout-v2.json`. Save paths use temporary-file replacement. Invalid layout/settings input falls back to defaults under the respective loader rules. Network history is the separate state log above. Source edits belong in the project, not these runtime data files.

This guide is based on source and installed-launcher inspection on 2026-10-22. It does not claim new runtime, connectivity, physical-touch or cursor regression tests. Claude's historical journal reports verification for several workflows, but hardware touch, original UTM pointer corruption, arbitrary external-window integration, some recovery cases and final visual acceptance remain unresolved. See WORK_TRACKER.md and WORK_PROVENANCE.md for the exact distinction between implemented, historically verified and finalized.
