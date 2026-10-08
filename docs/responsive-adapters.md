# Responsive adapters — Step 04 baseline verified

The GTK surface now consumes the pure solver when Auto Fit is selected or systems restore/minimize. The containing ScrolledWindow exposes the solver's canvas instead of scaling undersized content down. A backward-compatible automatic flag preserves pre-existing manual arrangements; manual Arrange disables automatic packing. Automatic layouts recompute on viewport changes. Transient attention is not yet connected (Step 05).

Existing widgets remain alive. System hero/metric sections, Files controls/preview and Utilities inputs change orientation/grid position in place with a 24px hysteresis band. Home instruments retain native typography. Expanded video now shares the inline preview MediaStream; opening a viewer does not deliberately restart playback or override paused state. Closing/unmapping still pauses the stream, and the explicit GTK test verifies a prepared paused stream retains its seek position when shared with a second viewer. Full workflow continuity remains a separate acceptance check.

The overview activation controls have semantic descriptions of their actual status/content. Descriptions update only when changed and are not live-announced every telemetry tick. This provides a baseline for custom drawings but does not replace Step 14's screen-reader/touch validation or fully interactive home controls.

Validation and unresolved acceptance are recorded in WORK_TRACKER.md. Do not label this step finalized merely because the initial 32 tests passed: focused/narrow/media UI checks and preservation review remain.

The bottom shortcut strip scrolls horizontally when constrained; Start, connection indicator and clock remain outside the strip. Status text may ellipsize, with full network details in Network. Exact allocation checks cover all six pages at wide, narrow, short and tall sizes within the actual VM work area. See responsive04/verified.log in the exchange for results. These checks do not establish touch-only usability or full focus/home job-state continuity.

Final baseline evidence: continuity04/restoration.log verifies Files path/filter/selection/metadata/scroll, remembered text-field focus through Console and Desktop transitions, and a real read-only diagnostic job completing while hidden. Input memory uses weak widget references and clears focus before reparenting. Pre-allocation focused width is clamped positive. Later workflow/touch/media/motion acceptance remains in Steps 06, 09, 14 and 20.
