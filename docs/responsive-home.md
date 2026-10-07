# Responsive LCARS home instruments

Home panels now render dedicated instruments at native pixel sizes instead of shrinking complete application pages. The visual hierarchy uses condensed LCARS lettering, amber values, lavender section strips, coral labels, and minimal black spacing. The existing compact header, exit control, taskbar, spatial zoom, arrangement modes, and adaptive background emblem remain.

## Panel behavior

- System: large clock and uptime, CPU history, and CPU/RAM/home-volume gauges expand into the available panel height.
- Files: current path and real directory rows; size/type details appear when width permits. The row count identifies overflow.
- Applications: actual installed or filtered applications, with a second column on wider panels.
- Network: live connectivity, receive/send rates, and the shared speed-test result; additional diagnostics and history appear with more space.
- Utilities: the actual calculator result and diagnostic status/report.
- Preferences: current workspace, alert sound, and layout controls, with additional fields when space permits.

Selecting a panel still zooms into its full controls. Home views share the original widgets and live telemetry; they do not launch network tests or diagnostic scans themselves. Files and applications support scrolling through rows. Other instruments have vertical overflow only below their minimum readable height. Touch swipes are separated from tap-to-focus and disabled during arrangement.

## Validation

Fedora build, formatting, strict Clippy, and all 16 tests passed. Tests cover Unicode-safe truncation, word wrapping, application-column breakpoints, existing geometry/packing/zoom/swap behavior, and service logic. Visual checks covered three-panel and six-panel layouts. Installed applications and actual directory entries populated correctly. Entering 2 + 3 in the full Utilities page displayed 5 in its home instrument, confirming shared state.

UTM automation did not reliably deliver pointer scrolling, so physical touch/swipe and wheel behavior still need a hands-on check. The original user arrangement was saved before temporary six-panel checks and restored afterward.
