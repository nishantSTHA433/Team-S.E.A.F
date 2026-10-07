# LCARS implementation work tracker

Maintainer: Codex. User-visible, assistant-maintained; the user does not need to update this file. Explicit user corrections always take precedence. This file is the authoritative execution tracker; historical docs are supporting context.

Authoritative location: /home/fedora/RustroverProjects/lcars-desktop/WORK_TRACKER.md
Updated: 2026-10-07

## Mandatory checkpoint before changes

1. Before any project change in every response, read this file from the Fedora project and inspect the current working tree. This includes code, assets, configuration, documentation, dependencies, and test fixtures. Read-only investigation may precede the checkpoint update.
2. Before making those changes, save CURRENTLY WORKING ON against the exact step/subtasks intended for this response. Fill the active checkpoint below with scope, files, verification, and the next concrete action. Updating this tracker itself is the checkpoint operation.
3. Work in dependency order. Split a large step into stable subtask IDs; never silently broaden scope. Record newly discovered work and dependencies here before implementing it.
4. Update the checkpoint immediately after meaningful milestones, failures, scope changes, or verification. Keep an exact next action so interruption recovery does not depend on chat memory.
5. At response end, record changes, evidence, remaining work, and handoff state. Never mark FINALIZED without the step's acceptance gate. A successful build alone is not visual or touch validation.
6. On resume, verify actual files, git diff/status, runtime state, and evidence before continuing. An interrupted task remains CURRENTLY WORKING ON until reconciled. Do not blindly rerun installation scripts or overwrite user changes.
7. Preserve unrelated user work. Stage only explicit task files. Keep the Mac export in sync after validated guest changes; Fedora remains authoritative. No credentials belong in this file.
8. Do not mark the overall project complete until Step 20 passes. Unverified hardware behavior remains explicitly unverified.

## Status meanings

- NOT STARTED: remaining work has not begun; existing partial functionality may already exist.
- CURRENTLY WORKING ON: changes are planned/underway; checkpoint must be populated.
- IMPLEMENTED: changes exist, but verification or review remains.
- VERIFIED: relevant checks passed and evidence is recorded.
- FINALIZED: acceptance gate passed, documentation/state saved, and no known required work remains within this step.
- BLOCKED: name the concrete dependency, what was attempted, and what permits progress. Continue independent authorized work where possible.

## Active checkpoint

- Current task: Step 04a–04e CURRENTLY WORKING ON; begin with responsive GTK adapters, safe native-size viewport integration and media-stream continuity.
- Steps 01–03 FINALIZED. Step 03 evidence: /mnt/macos/layout03/build.log, build/strict Clippy/31 tests passed; all 64 panel subsets across eight sizes validated. Narrow overflow bug fixed before finalization.
- Intended files: src/console.rs, src/ui.rs, src/tools.rs, src/overview.rs, docs/responsive-adapters.md; WORK_TRACKER.md.
- Scope: adapt existing persistent widgets without rebuilding/resetting application state; preserve saved manual arrangement. No claim that Step 05 automatic task policy or Step 06 animation polish is complete.
- Next action: inspect and adapt viewport and focused content; reflow component rows at narrow sizes; share media stream between preview/viewer; verify build and UI states.
- Resume: inspect /mnt/macos/responsive04/ staged changes and guest diff before reapplying. Physical touch and external app/session work remain open.

## Existing baseline — preserve, do not call final

Six built-in systems: System, Files, Network, Applications, Utilities, Preferences. Existing live telemetry, connection alerts, bounded/cancellable speed test, app discovery/launch, drive aliases, Trash, image/video preview, Desktop/Console setting, taskbar/Start, compact identity header and Exit, zoom/minimize/restore, Basic/Advanced Arrange, saved geometry, adaptive emblem and native-pixel home summaries. Latest recorded Fedora build and strict Clippy passed with 16 tests. Visual passes through 0f1450f are implemented; end-to-end final acceptance remains outstanding.

## Ordered execution checklist

### Step 00: Establish the work tracker
Status: FINALIZED. Dependencies: none.
- 00a: Write ordered plan, state protocol, current checkpoint and acceptance coverage.
- 00b: Install in project root and add an agent-discovery pointer without replacing existing instructions.
- 00c: Read back installed files, verify integrity, finalize this documentation task.
Gate: tracker and discovery rule present in Fedora, readable by user, accurate statuses and next action.

### Step 01: Make the shared workspace and content model
Status: FINALIZED. Dependencies: 00.
- 01a: Audit source, working tree, existing docs and data ownership; map partial features to this plan.
- 01b: Define panel identity, visibility, focus, activity, selected content and durable versus temporary state.
- 01c: Define minimum/preferred dimensions, aspect preference, protected controls and content priorities for each system.
- 01d: Define a single event vocabulary for focus, return, minimize, preview, diagnostics, viewport and keyboard changes.
Gate: explicit model covers all six systems and state transitions without resetting work; record files and decisions.

### Step 02: Prove the platform and input boundaries early
Status: FINALIZED. Dependencies: 01.
- 02a: Document exact title/taskbar/maximize/minimize/content-tap behavior; no essential double taps or hover.
- 02b: Determine GTK/UTM touch, keyboard, accessibility and rendering constraints with small focused checks.
- 02c: Prototype feasibility of native external-window integration and Fedora shell/session boundary. Do not promise internal reflow of arbitrary third-party apps.
- 02d: Define supported external-app behavior and record any unresolved requirement; do not silently reduce scope.
Gate: architecture is grounded in supported input/platform capabilities before broad rendering work.

### Step 03: Build the content-aware layout solver
Status: FINALIZED. Dependencies: 01, 02.
- 03a: Allocate rectangles using readable minima, active-task needs, supporting panels, emblem and viewport.
- 03b: Prefer roughly equal home areas when idle; allow asymmetric task-driven expansion.
- 03c: Minimize wasted gaps while retaining readable grouping and touch separation.
- 03d: Add stable tie-breaking and thresholds to prevent layout oscillation.
- 03e: Handle 0–6 systems, extreme aspect ratios and minimum supported viewport; define overflow fallback.
Gate: deterministic layouts satisfy bounds, visibility, content minima and stability across a size/panel matrix.

### Step 04: Update responsive rendering and state preservation
Status: CURRENTLY WORKING ON. Dependencies: 03.
- 04a: Implement compact, standard, expanded and focused compositions selected by width AND height.
- 04b: Reflow sections/columns; preserve native readable text instead of shrinking whole pages.
- 04c: Preserve file selection, path, search, scroll, media position, test progress and input focus.
- 04d: Keep essential actions visible; use scrolling for genuinely long content, avoid nested scroll traps.
- 04e: Provide accessible semantics for custom-drawn status and controls.
Gate: every system works in narrow/wide/short/tall/focused rectangles with state retained.

### Step 05: Add automatic attention and space allocation
Status: NOT STARTED. Dependencies: 03, 04.
- 05a: Expand for selected media, diagnostics, speed test and text input; keep supporting context useful.
- 05b: Release temporary space and restore prior composition when the task ends.
- 05c: Freeze geometry around active touches/typing; routine telemetry must not move windows.
- 05d: Alerts remain visible without stealing focus or destroying the current task.
Gate: actions produce useful space automatically with no repeated oscillation or unnecessary manual arrangement.

### Step 06: Finalize coordinated motion and hit testing
Status: NOT STARTED. Dependencies: 04, 05.
- 06a: Animate bounds, content transitions and emblem together; prototype 200–350 ms timing, then measure.
- 06b: Support interruption/reversal from current geometry; clip all moving content correctly.
- 06c: Keep touch targets aligned; hidden systems receive no input.
- 06d: Add reduced motion; preserve state during rapid focus/minimize/return sequences.
Gate: no stale frames, detached controls, input leaks or lost state during stress sequences.

### Step 07: Build and apply the unified LCARS visual system
Status: NOT STARTED. Dependencies: 04, 06.
- 07a: Consolidate palette, fonts, type hierarchy, rails, curved elbows, caps, segments, icons and interaction states.
- 07b: Replace conflicting accumulated CSS and generic closed boxes with coherent open-sided structures.
- 07c: Apply styling to home, focused systems, menus, dialogs, lists and error states.
- 07d: Keep thin visible elements backed by generous touch targets; distinguish decoration from controls.
Gate: screenshot comparison shows one consistent reference-led language across all views, with readable text.

### Step 08: Finalize adaptive emblem composition
Status: NOT STARTED. Dependencies: 03, 06, 07.
- 08a: Prefer central exposure; allow decorative overlap behind opaque panels without covering important content.
- 08b: Resize/reposition smoothly; switch stacked/horizontal identity composition as needed.
- 08c: Avoid oversized empty reservations and jumping between similar gaps; retain small header identity in focus.
Gate: emblem remains integrated through open/minimize/preview/reflow scenarios without hiding controls or text.

### Step 09: Complete Files and media as the first adaptive workflow
Status: NOT STARTED. Dependencies: 04–08.
- 09a: Responsive list, single-tap folder navigation, selection, path/search and custom LCARS type icons.
- 09b: Stable C:/D:/E: mapping, mounted-volume updates and missing-drive recovery.
- 09c: Image side preview plus properties; stack when narrow; allocate space automatically; one-tap larger viewer.
- 09d: Video preview, play/pause, seek, duration, expansion and playback-state preservation.
- 09e: Open and recoverable Trash with feedback; loading, errors, unsupported media and permissions.
Gate: rapid file selection shows the correct preview; focus/return and reflow preserve context; all actions work with touch.

### Step 10: Complete System and Diagnostics
Status: NOT STARTED. Dependencies: 04–08; use patterns proven in 09.
- 10a: Responsive useful CPU/memory/storage/uptime/history instrumentation based on actual guest hardware.
- 10b: Structured process list, sorting/filtering where useful and readable diagnostic findings.
- 10c: Compact summaries grow into details; show unavailable sensors honestly.
- 10d: Clear running/cancelled/completed/failed states; separate inspection from disruptive actions.
Gate: useful inspection without terminal-style dumps; actual data, readable actions and retained diagnostic context.

### Step 11: Complete Network and alert workflows
Status: NOT STARTED. Dependencies: 04–08; use patterns proven in 09.
- 11a: Persistent internet/local-only/offline/limited/captive state where detectable.
- 11b: Transition-driven visual/audio alerts, no duplicate alarm spam, clear recovery and sound preference.
- 11c: Responsive speed-test progress/results/cancel/failure with transfer disclosure.
- 11d: Structured adapter/route/DNS/reachability diagnostics and explicit supported repair actions with results.
Gate: real disconnect/reconnect, failure and cancellation tests pass without misleading status or lost browsing context.

### Step 12: Finish Start, taskbar, Preferences and workspace modes
Status: NOT STARTED. Dependencies: 07, 09–11.
- 12a: Compact header, identity and rightmost Exit; no permanent left navigation rail.
- 12b: Start search and all-system access; active/open/minimized taskbar states and touch overflow.
- 12c: Keep fullscreen and infrequent controls in settings/menu; Exit distinct from OS power actions.
- 12d: Preserve saved Desktop/Console toggle and consistent preferences; desktop option is a desktop.
- 12e: Finish calculator and remaining Utilities controls using shared responsive patterns.
Gate: every system can be reached/restored with touch and shell controls remain compact at supported widths.

### Step 13: Finalize optional manual arrangement
Status: NOT STARTED. Dependencies: 03, 05, 06, 12.
- 13a: Enter Basic first; drag/drop swap plus accessible tap alternative.
- 13b: Basic dropdown offers Advanced; Advanced unlocks move/resize and precise touch alternatives.
- 13c: Done, Cancel and Auto Fit; geometry locked outside Arrange.
- 13d: Persist manual preference; temporary attention returns to it; recover gracefully after viewport changes.
Gate: manual override works without becoming a requirement for routine tasks or fighting automatic layout.

### Step 14: Finalize whole-app touch and accessibility
Status: NOT STARTED. Dependencies: 02, 09–13.
- 14a: Verify tap/scroll/drag separation on actual hardware; keyboard/mouse checks supplement, not replace, touch.
- 14b: Keyboard-aware layout, text-entry access and focus restoration throughout.
- 14c: Accessible names/roles, focus order, pressed feedback, contrast and reduced motion.
- 14d: Eliminate hover-only, double-tap and right-click dependencies.
Gate: all primary journeys complete with touch only; hardware-unverified items stay open.

### Step 15: Complete native Fedora application integration
Status: NOT STARTED. Dependencies: early boundary proof in 02; 12, 14.
- 15a: Reliable discovery/search/launch and useful LCARS icons; progress/failure handling.
- 15b: Implement the supported running-app/return-to-console and external-window strategy from 02.
- 15c: Verify spatial integration only where actually supported; explicitly track any unsatisfied external-window requirement.
Gate: native apps remain reachable and return paths work; built-in versus external capabilities are accurately documented.

### Step 16: Complete persistence and failure recovery
Status: NOT STARTED. Dependencies: 09–15 (basic state preservation already required in 04).
- 16a: Versioned durable settings/layouts with atomic writes and invalid-data recovery.
- 16b: Removed drives/files, missing codecs, unavailable telemetry, launch failures and permission errors.
- 16c: Restart during activity; do not silently restart costly tests or unexpected playback.
Gate: each failure has a usable recovery path without a terminal; durable state survives relaunch.

### Step 17: Resolve rendering/cursor reliability and tune performance
Status: NOT STARTED. Dependencies: 06, 14–16; fix blockers earlier whenever encountered.
- 17a: Reproduce original pointer disappearance/corruption across app/UTM/macOS transitions.
- 17b: Test menus, media, fullscreen, display changes and suspend/resume.
- 17c: Measure animation/input/resource behavior in Fedora UTM; tune without breaking touch targets or fidelity.
Gate: repeatable test matrix passes; renderer workaround alone is not proof; record environment and remaining limitations.

### Step 18: Finish installation and optional desktop/session use
Status: NOT STARTED. Dependencies: 02, 15–17.
- 18a: Repeatable install/launch/update path; optional startup/fullscreen configuration.
- 18b: Recovery and normal Fedora desktop access; distinguish Exit, logout and power actions.
- 18c: Multi-display/scaling, suspend/resume and supported session behavior.
Gate: LCARS is usable as a primary interface without trapping the user or removing recovery access.

### Step 19: Finalize visual and motion polish
Status: NOT STARTED. Dependencies: 08–18.
- 19a: Compare home/focused/media/diagnostic/menu states with the reference at actual viewport sizes.
- 19b: Tune density, curved rails, typography, icons, motion and configurable sound.
- 19c: Remove generic leftovers, misleading decorative controls, flashes and awkward empty regions.
Gate: consistent finished appearance while idle and in motion; no regression in readability or operation.

### Step 20: Run final acceptance and close the project
Status: NOT STARTED. Dependencies: 01–19 FINALIZED or explicitly user-approved scope revisions.
- 20a: Run the scenario matrix below and relevant automated checks; attach real evidence.
- 20b: Recheck original BRD and subsequent user requirements; resolve omissions rather than silently declaring them optional.
- 20c: Review source changes, documentation, install/recovery instructions and saved checkpoint.
- 20d: Record final version/commit, remaining user-approved limitations and completed-state handoff.
Gate: no unresolved required work. Subjective plan scores never substitute for measured/observed acceptance.

## Final scenario matrix (all currently pending final validation)

All six systems together; image selection/side properties; video play/seek/expand; focus/return preserving state; minimize/restore repacking; speed test while browsing; offline/recovery alerts; keyboard opening; repeated viewport changes; Basic swaps; Advanced move/resize/Cancel; saved Desktop/Console setting; restart and damaged state; removed drives; unsupported media; native app launch/return; UTM re-entry/cursor; touch-only journey; reduced motion/accessibility; optional session recovery.

## Requirements cross-check

- Reference and BR-01/02/04: 07, 08, 19; Okuda-era reference, no DS9/Kelvin substitution.
- Touch BR-03: 02, 06, 09–14; non-touch testing also supported.
- Hardware BR-05: 10, 17; actual guest data, no fabricated sensors.
- Native applications BR-06: 02, 12, 15; custom Files/Diagnostics explicitly covered in 09/10.
- Network BR-07–10: 11; persistent status, audio/visual changes, troubleshooting and speed test.
- Files BR-11–14: 09; stable drive aliases, metadata, Open/Trash, custom icons and media extension requirements.
- Spatial/AI-like adaptation: 01, 03–06, 08; equal idle prominence, contextual space, stable behavior, no manual work required.
- Compact header/Exit/no left rail/Start/taskbar: 12.
- Single activation, active-title return, maximize/minimize: 02, 06, 12.
- Basic/Advanced Arrange and locked normal geometry: 13.
- Desktop/Console setting: 12, 16.
- Pointer corruption and VM compatibility: 17.
- Fedora-only eventual desktop role: 02, 15, 18.

## Per-response journal template

Date / step and subtask IDs:
Before-change status and intended scope:
Files and relevant baseline commit:
Changes actually made:
Verification commands / observed UI / evidence paths:
State: CURRENTLY WORKING ON | IMPLEMENTED | VERIFIED | FINALIZED | BLOCKED
Known failures / unverified behavior:
Exact next action / safe resume instructions:
Commit/export reference (if applicable):

## Journal

2026-10-07 / Step 00: Tracker drafted and second-pass reviewed for dependency order, coverage, unambiguous states and interruption recovery. Installation/read-back completed; Step 00 finalized. Quality assessment is a planning review, not evidence that implementation is finished.

2026-10-07 / Step 01a–01d CURRENTLY WORKING ON: checkpoint saved before source changes; audit and shared model are the authorized scope.

2026-10-07 / Step 01 FINALIZED: Fedora build and strict Clippy passed, 26/26 tests, successful visual launch. Step 02 CURRENTLY WORKING ON: input/platform feasibility and interaction contract.

2026-10-07 / Step 02 FINALIZED: measured platform/input capabilities and documented integration boundary; no system settings changed. Step 03 CURRENTLY WORKING ON: content-aware solver.

Step 03 verification update: narrow-viewport test caught avoidable 8px horizontal overflow. Restricting candidates to viewport width (or largest unavoidable single-panel minimum); then re-running full validation.

2026-10-07 / Step 03 FINALIZED: solver and invariant matrix passed, 31 total tests, strict Clippy/build clean. Step 04 CURRENTLY WORKING ON: live responsive adapters and preservation.
