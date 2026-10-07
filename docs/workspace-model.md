# Shared workspace model — Step 01

## Source audit and current ownership

Baseline: 0f1450f. No tracked user source changes were reported in the guest audit; untracked IDE/setup/backup directories are preserved. AGENTS.md and WORK_TRACKER.md were previously created by this assistant.

- console.rs owns saved normalized rectangles/hidden flags, focus, Arrange state, interpolation, and input covers. Layout still uses fixed slot patterns. It is NOT migrated to the new reducer in Step 01.
- ui.rs builds persistent full-page GTK widgets, navigation/shortcuts, Files path/search/selection, media preview and native-app launching.
- overview.rs reads references to those widgets and InstrumentData for native-pixel home summaries. It is still a presentation bridge rather than an independent application model.
- tools.rs owns calculator results, asynchronous diagnostics, speed-test progress and cancellation. network.rs runs network operations; services.rs owns telemetry, mount aliases and preferences.
- Media preview has its own selection/generation token. Expanded video currently creates a separate player and inline video pauses on unmap: exact playback continuity is outstanding Step 04/09 work, not a verified property of the existing app.

## Introduced contracts

src/lib.rs exposes a GTK-independent workspace module. The binary consumes its stable catalogue, retaining route order and names for saved-layout compatibility. Public model APIs are intentionally reusable by later GTK adapters; they do not execute commands, perform network I/O or launch applications.

PanelId replaces undocumented identities in the shared vocabulary. Each of the six systems declares initial logical-pixel minimum/preferred content sizes, preferred shape, protected information roles, and collapsible secondary content. These are design inputs for Step 03, not measurements or enforced layout constraints yet. Measurements in Step 02/04 may revise them with evidence.

Workspace separates durable minimized flags from transient focus, attention, input contacts, keyboard occlusion, arrangement mode, per-panel activity and content. Existing saved geometry/settings formats are unchanged. ContentState provides location, selection/media type, search, scroll, playback and result snapshots. Actual GTK widgets remain authoritative until their explicit adapters are migrated in Step 04; do not maintain a second unsynchronized live source of truth.

## Event contract

- Activate: restore and focus; activating the focused panel returns home. Ignored during arrangement.
- Maximize: restore and focus explicitly. Minimize: hide a panel; clear only matching focus/attention/keyboard ownership.
- Home: return to composition without clearing panel content or ongoing job state.
- ReplaceContent and SetActivity: update task state without requesting geometry changes. Unsupported activity/panel combinations are ignored.
- RequestAttention/ReleaseAttention: request temporary home emphasis without changing durable minimized state or stealing explicit focus; minimized panels do not restore automatically.
- Viewport: update logical available dimensions. Keyboard: report owner and occlusion, saturating available height at zero rather than underflowing.
- ContactBegin/End: track independent contacts; defer geometry until the last contact ends. Duplicate or unknown end events cannot release another contact.
- BeginArrange always selects Basic; AdvancedArrange is valid only inside Arrange. EndArrange permits automatic layout again. Manual geometry/swap/cancel transactions remain Step 13 responsibilities.

Effects report changed state, required reflow and durable-state changes. Only the renderer/persistence adapter acts on these effects. Ordinary updates cannot move windows. Repeated identical events do not restart transitions. While typing, unrelated reflow waits; keyboard avoidance itself is allowed after active contacts finish. Model state can advance while geometry is deferred: the future renderer must preserve displayed hit regions until its transition is applied (Step 06).

## Migration sequence

Step 02 checks real input/platform boundaries. Step 03 consumes content contracts in a pure solver. Step 04 migrates each GTK state owner once, with adapters and preservation checks. Step 05 connects activity to explicit attention policy. Step 06 unifies rendering/hit-testing transitions. Do not infer that introducing this model already delivers those later behaviors.

## Validation

Meaningful model tests cover catalogue/contract completeness, focus/minimize/restore retention, hidden visibility, no focus theft by background work, invalid activities, transient attention versus durable restoration, independent contacts, keyboard occlusion/deferred avoidance, Basic/Advanced entry and idempotent events. Existing application regression tests remain required. Final results are recorded in WORK_TRACKER.md and /mnt/macos/model01/build.log.
