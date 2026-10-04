# Console and Desktop modes — October 3

The legacy single-page design is called **Desktop mode**. Preferences → Console Mode switches between it and the touch console; the preference persists.

## Console controls

- **ALL SYSTEMS:** zoom out to six live system previews. The previews are read-only; their large title bands and preview surfaces select the system for use.
- **Select a system:** expand it to full working size. Its original state and functionality remain intact. The left rail also selects systems.
- **ARRANGE / DONE:** show/hide layout editing. Tap a system to select it (amber), drag its title band to move it, or drag its ↘ handle to resize it. Large arrow and SMALLER/LARGER controls provide discrete adjustments.
- **AUTO FIT:** restore the default non-overlapping arrangement around a central empty region.
- Geometry uses screen-relative coordinates, bounds checking and grid snapping. Adjustments save atomically in ~/.config/lcars-desktop/console-layout.json. A malformed layout falls back to defaults. Manual arrangements can overlap; AUTO FIT restores separated regions.
- The background emblem chooses available space using a sampled occupancy map. It yields to focused content.

## Verified this round

Build, strict Clippy and ten unit tests passed. Geometry tests cover invalid/offscreen values, serialization/snapping, and non-overlapping defaults. The VM shows all six scaled systems, focus expansion, Arrange controls and saved button-driven movement/resizing. Physical touch and gesture accuracy remain acceptance work; UTM pointer automation did not establish a reliable drag result.

## Next work

1. Animate overview/focus transitions while honoring reduced motion. Current zoom changes are immediate.
2. Remove duplicate inner page chrome in Console mode and create dedicated compact instrument compositions instead of relying solely on scaled desktop pages.
3. Improve logo sizing in irregular gaps, automatic collision resolution, and optional independent width/height touch controls.
4. Verify real touch dragging/resizing, pinch gestures if added, on-screen keyboard, rotation, and multiple resolutions. No keyboard action should be required.
5. Complete all BR-01–BR-14 acceptance from requirements.md: reference-fidelity review, physical telemetry, native app launch handling, network failure/portal/audio cases, safe disposable-file Trash recovery, removable drives, per-format icons and diagnostics repair flows. Preserve image/video inspection.

This remains a Fedora app inside GNOME. External native applications are not embedded or rethemed; desktop/session replacement and supported-Fedora packaging remain separate release work.
