# LCARS reference-fidelity roadmap — October 3

## Completed this round

- Narrow Nimbus Sans Narrow headings using the font already installed in Fedora.
- Original vector laurel frame around the star field, segmented instrument chrome, square meters, live 60-sample CPU history, file-table and diagnostic headings.
- Image preview beside properties with source pixel dimensions; aspect-preserving larger viewer. Thumbnail decoding runs in a worker, rejects images over 32 MB or 80 megapixels, and ignores obsolete results after selection changes.
- Video poster preview with explicit playback and larger viewer; resolution and duration properties. Unsupported media reports errors and retains external OPEN. Playback pauses when its preview is unmapped.
- Saved Preferences → Workspace Mode switch. Off retains single-page navigation; on opens tools together. CLOSE hides a workspace panel without discarding its state. Narrow windows use a vertical layout; sufficiently wide windows use two columns. Each panel scrolls independently when its contents need more room.
- Background emblem changes size/position with panel occupancy. Compact workspace overview yields its large foreground emblem to telemetry. Alt+1–6 opens tools.

## Next visual work, in priority order

1. **Panel-specific adaptive layouts:** remove duplicated page/panel title strips in Workspace mode, compress secondary status text, wrap toolbars and turn the overview into the reference's narrow instrument cluster. Retain readable text and touch-sized actions. The initial workspace uses scrolling and does not yet fit every tool's entire contents simultaneously.
2. **Workspace interaction:** add focus/bring-into-view for an already open panel, user-adjustable proportions, saved open-panel arrangement, and consistent active/open indicators. Stress-test repeated open/close and mode switches during diagnostics, speed tests and playback.
3. **Background emblem placement:** find actual unused rectangles between panels, then move and scale the emblem into those spaces. Current placement is a simple center/lower-right rule, not geometry-aware placement.
4. **Reference composition:** match the screenshot's left clock/system instrument, larger central status/emblem, right diagnostic frame and continuous bottom task strip. Compare normal/fullscreen and several window sizes side-by-side with the supplied image.
5. **File explorer:** add sortable modified-date/type columns, breadcrumbs and a complete LCARS file-format icon family. Make media preview/properties sizing adjustable; test corrupt files, rapid selection changes, very large images, diverse codecs and unavailable media.
6. **Diagnostics:** convert the text report into framed adapter, route, DNS and process sections; add real memory/traffic histories with age/scale labels. Provide clear repair options with previews before changing network configuration.
7. **Calculator/application launcher:** implement the screenshot-like keypad and compact shortcut arrangement. Native launched applications still use their own windows; they are not embedded or rethemed by this app.
8. **Shared visual components:** unify Utilities, Preferences, media viewer and dialogs with the other LCARS frames; bundle and license a tested condensed font; review focus/hover/disabled states and high-DPI rendering.

## Requirements and remaining acceptance

The original BRD remains authoritative; user additions extend it. Keep the approved Okuda-era direction, excluding DS9/Kelvin, and target Fedora.

| Requirement | Current implementation / remaining acceptance |
|---|---|
| BR-01/02/04: aesthetic and visual quality | Reference-led palette, frames, emblem and typography; final composition and user visual acceptance pending |
| BR-03: touch-first | Large action targets; physical touch, on-screen keyboard, scaling and keyboard-only testing still required |
| BR-05: live hardware data | Guest CPU/memory/uptime/storage/traffic; physical sensors and real-PC validation pending |
| BR-06: native applications | GIO discovery/filter/launch; broader launch/failure testing pending |
| BR-07/08: online indicator and alerts | Live NetworkManager state, persistent indicator/banner, native notification and optional sound; loss/restoration previously tested; broader portal/limited/audio scenarios pending |
| BR-09: troubleshooting | Read-only diagnostics and Fedora settings link; structured guidance and repair workflows pending |
| BR-10: speed test | Bounded 10 MB download/2 MB generated upload, cancellation and timeouts; expand failure-path acceptance |
| BR-11: C:/D:/E: drives | Persistent drive mapping and hotplug refresh; more removable-drive testing and pagination beyond 2,000 files pending |
| BR-12/13: properties/Open/Delete | Metadata, explicit Open and confirmed recoverable Trash; disposable-file recovery, permissions and symlink acceptance pending |
| BR-14: custom graphics | File/folder graphics and vector emblem; complete per-format icons pending |
| Added: media inspector | JPEG preview and generated WebM preview/player visually checked; expanded image viewer and broader formats need additional acceptance |
| Added: switchable adaptive workspace | Saved mode toggle, state-preserving panel reparenting, tiling and initial emblem behavior implemented; refined geometry and wider-screen acceptance pending |

Build, formatting, strict Clippy and seven unit tests passed. Manual checks covered the running overview, image dimensions/preview, WebM poster/expanded viewer, opening Workspace mode and returning to single-page mode. Full physical touchscreen and compositor acceptance is not claimed.

## Release work

Plan a supported Fedora migration, reproducible package, startup/recovery behavior, target-architecture testing and continued pointer regression checks. Preserve the current working VM during migration. The app still runs within GNOME; full desktop/session replacement is unfinished. The network monitor currently requires the app to remain running.
