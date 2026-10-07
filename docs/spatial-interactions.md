# Spatial LCARS console

## Interaction

One activation of a system title, home preview, Start entry, or taskbar shortcut focuses it with a 300 ms eased zoom. Activating the same system again returns home. The lavender square maximizes its system; the amber minus minimizes it. Other systems become invisible and non-targetable while one is focused. Minimized systems reopen from Start or a taskbar shortcut. Native Fedora applications still launch in their own windows; these spatial controls govern the six built-in LCARS systems.

Start and Preferences contain Home, Arrange, and Auto Fit. Fullscreen lives in Preferences. The compact top strip contains the LCARS identity, emblem, and a single Exit button at the far right. Arrangement controls appear only during arrangement.

Arrange always begins in Basic mode. Drag a title or preview onto another system to exchange their rectangles immediately on drop. Touching two different systems also swaps them, as an accessible alternative. The Basic dropdown switches to Advanced, which unlocks free dragging, corner resizing, and touch-sized move/resize buttons. Outside Arrange the system rectangles are locked. Done saves the arrangement.

Single-touch folder activation replaces double-click navigation. Selecting a file shows its preview/properties; the separate Open control launches it.

## Composition

Automatic packing responds to the number of visible systems. With six systems, the rectangular areas differ by less than 5%, using taller outer panes and wider center panes. Application rectangles plus the reserved logo rectangle cover over 95% of normalized workspace area for every nonempty system count. Small gutters remain for LCARS edges and touch separation. Auto Fit restores this composition after free arrangement.

The wallpaper finds a large empty rectangle with a preference for the center. It switches between a horizontal emblem/status composition and a stacked composition as available space changes. The emblem extends past the gap underneath opaque application surfaces, which protect application text and controls. Maximized systems own the workspace, while the small header emblem remains visible.

Layout and minimized state are saved atomically in ~/.config/lcars-desktop/console-layout-v2.json. The original console-layout.json remains untouched.

## Validation

Fedora build, strict Clippy, and 13 tests passed. Tests cover packing, reserved logo space, geometry bounds, zoom interpolation, and swap hit testing that excludes hidden systems and the source. UI checks exercised focus/return zoom, maximize, minimize/restore, Basic position exchange, Advanced movement/resizing, and the relocated Start actions. UTM automated mouse/touch delivery was inconsistent: keyboard activation verified these actions, but physical drag-and-drop and touchscreen feel still need a hardware/manual check.
