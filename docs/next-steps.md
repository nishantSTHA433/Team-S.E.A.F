# LCARS next steps — reference fidelity and BRD acceptance

Completed in this round: original vector laurel frame around the star field; genuine rolling CPU trace (60 recent samples, fixed 0–100% scale); file-list column strip; diagnostic section strip; square instrument meters. Existing 48-pixel action targets remain.

## Next visual work, in order

1. Typography (BR-01/02/04): bundle a redistributable tall condensed face with its license; apply it to LCARS branding, panel titles and instrument numbers. Keep file paths and diagnostic text legible. The current fallback font remains wider than the reference.
2. Overview composition (BR-04/05): arrange a narrow system/clock instrument at left, larger central emblem and status, and a right-hand diagnostic frame. Make layout adapt to smaller VM windows without clipping or shrinking touch targets.
3. Shared frame components (BR-01/04): unify segmented title bars, elbow corners, menu strips and borders across Files, Network, Applications, Utilities and Preferences. Utilities still has older page chrome.
4. File explorer (BR-11/12/14): add aligned modified-date and type columns with sortable headings, breadcrumbs, and distinct document/image/archive/application icons. Keep real paths, persistent drive labels, explicit Open and recoverable Trash.
5. Diagnostics (BR-05/09): show actual CPU/memory and network histories in compact framed instruments; expose sample age and missing/stale data explicitly. Build structured diagnostic findings and useful next actions from the current text report.
6. Calculator and launcher (BR-06/04): match the reference keypad and application-shortcut arrangement with functional touch targets and keyboard navigation. Native launched apps currently retain their own windows; custom Files and Diagnostics remain within LCARS.
7. Footer and window behavior (BR-03/04): improve task selection states and date/clock placement; define how multiple tools remain accessible. Preserve fullscreen escape and pointer visibility. Desktop/session replacement is still unfinished and must be developed separately from the current GTK app.
8. Side-by-side visual acceptance: capture each page at normal and fullscreen sizes; compare font proportions, spacing, panel geometry, palette and focus states with the supplied reference. Preserve the requested Okuda-era direction and exclude DS9/Kelvin elements.

## Functional acceptance still required

- BR-03: physical touch and on-screen keyboard tests; small-window, keyboard-only and scaling checks.
- BR-05: physical hardware telemetry validation; no invented readings for sensors absent from UTM.
- BR-06: application discovery and launch failure handling across installed apps.
- BR-07/08: repeated online/offline, limited and portal transitions; audio/mute handling and notifications; app must currently remain running.
- BR-09: guided network repair actions with clear previews; current diagnostics are read-only and link to Fedora settings.
- BR-10: speed-test cancellation/timeouts/error states and rate interpretation; preserve explicit bounded transfer disclosure.
- BR-11: multiple/removable drives, stable labels and removal during browsing; directory pagination beyond 2,000 entries.
- BR-12/13: disposable-file metadata/open/Trash/recovery tests, permissions and symbolic links; do not use personal files for deletion tests.
- BR-14: complete the LCARS icon family and verify high-DPI rendering.
- Packaging/session integration: supported Fedora migration, reproducible installation, startup/recovery behavior and testing on target architecture. Keep the current working Fedora VM intact during that future migration.

See requirements.md for the complete BR-01–BR-14 mapping. These items are a remaining-work list, not a claim that the full desktop replacement is finished.
