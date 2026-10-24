# LCARS implementation work tracker

Maintainers: Codex and Claude AI (continuation attributed per user instruction; see docs/WORK_PROVENANCE.md). User-visible, assistant-maintained; the user does not need to update this file. Explicit user corrections always take precedence. This file is the authoritative execution tracker; historical docs are supporting context.

Authoritative location: /home/fedora/RustroverProjects/lcars-desktop/WORK_TRACKER.md
Updated: 2026-10-22

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

- Current task: 2026-10-22 provenance and architecture documentation FINALIZED.
- Last Codex implementation: Step 07a shared palette/CSS foundation and Preferences cleanup; last committed baseline 460218f (Step 06). This documentation pass does not advance feature implementation.
- Later implementation and verification through October 12: credited to Claude AI per user instruction; source comments and docs/WORK_PROVENANCE.md identify mixed ownership.
- Changed: provenance comments only in mixed source/assets and installer; docs/WORK_PROVENANCE.md; docs/ARCHITECTURE_GUIDE.md; stale checklist statuses reconciled with existing journal.
- Verification: current source and installed desktop entry inspected; additions checked to preserve every pre-existing byte in annotated executable/style files. No runtime tests or source rebuild needed for comment-only changes; historical validation not repeated.
- Next action: follow the user's next request. Before further implementation, reconcile remaining 14/16/17/18 checks and 07/19/20 acceptance; do not silently mark those complete.

## Existing baseline — preserve, do not call final

Six built-in systems: System, Files, Network, Applications, Utilities, Preferences. Existing live telemetry, connection alerts, bounded/cancellable speed test, app discovery/launch, drive aliases, Trash, image/video preview, Desktop/Console setting, taskbar/Start, compact identity header and Exit, zoom/minimize/restore, Basic/Advanced Arrange, saved geometry, adaptive emblem and native-pixel home summaries. Latest recorded Fedora build and strict Clippy passed with 16 tests. Visual passes through 0f1450f are implemented; end-to-end final acceptance remains outstanding.

## Ordered execution checklist

### Step 00: Establish the work tracker
Status: FINALIZED. Dependencies: none.
Attribution: Codex baseline; later corrections credited to Claude AI where journaled.
- 00a: Write ordered plan, state protocol, current checkpoint and acceptance coverage.
- 00b: Install in project root and add an agent-discovery pointer without replacing existing instructions.
- 00c: Read back installed files, verify integrity, finalize this documentation task.
Gate: tracker and discovery rule present in Fedora, readable by user, accurate statuses and next action.

### Step 01: Make the shared workspace and content model
Status: FINALIZED. Dependencies: 00.
Attribution: Codex baseline; later corrections credited to Claude AI where journaled.
- 01a: Audit source, working tree, existing docs and data ownership; map partial features to this plan.
- 01b: Define panel identity, visibility, focus, activity, selected content and durable versus temporary state.
- 01c: Define minimum/preferred dimensions, aspect preference, protected controls and content priorities for each system.
- 01d: Define a single event vocabulary for focus, return, minimize, preview, diagnostics, viewport and keyboard changes.
Gate: explicit model covers all six systems and state transitions without resetting work; record files and decisions.

### Step 02: Prove the platform and input boundaries early
Status: FINALIZED. Dependencies: 01.
Attribution: Codex baseline; later corrections credited to Claude AI where journaled.
- 02a: Document exact title/taskbar/maximize/minimize/content-tap behavior; no essential double taps or hover.
- 02b: Determine GTK/UTM touch, keyboard, accessibility and rendering constraints with small focused checks.
- 02c: Prototype feasibility of native external-window integration and Fedora shell/session boundary. Do not promise internal reflow of arbitrary third-party apps.
- 02d: Define supported external-app behavior and record any unresolved requirement; do not silently reduce scope.
Gate: architecture is grounded in supported input/platform capabilities before broad rendering work.

### Step 03: Build the content-aware layout solver
Status: FINALIZED. Dependencies: 01, 02.
Attribution: Codex baseline; later corrections credited to Claude AI where journaled.
- 03a: Allocate rectangles using readable minima, active-task needs, supporting panels, emblem and viewport.
- 03b: Prefer roughly equal home areas when idle; allow asymmetric task-driven expansion.
- 03c: Minimize wasted gaps while retaining readable grouping and touch separation.
- 03d: Add stable tie-breaking and thresholds to prevent layout oscillation.
- 03e: Handle 0–6 systems, extreme aspect ratios and minimum supported viewport; define overflow fallback.
Gate: deterministic layouts satisfy bounds, visibility, content minima and stability across a size/panel matrix.

### Step 04: Update responsive rendering and state preservation
Status: FINALIZED. Dependencies: 03.
Attribution: Codex baseline; later corrections credited to Claude AI where journaled.
- 04a: Implement compact, standard, expanded and focused compositions selected by width AND height.
- 04b: Reflow sections/columns; preserve native readable text instead of shrinking whole pages.
- 04c: Preserve file selection, path, search, scroll, media position, test progress and input focus.
- 04d: Keep essential actions visible; use scrolling for genuinely long content, avoid nested scroll traps.
- 04e: Provide accessible semantics for custom-drawn status and controls.
Gate: every system works in narrow/wide/short/tall/focused rectangles with state retained.

### Step 05: Add automatic attention and space allocation
Status: FINALIZED. Dependencies: 03, 04.
Attribution: Codex baseline; later corrections credited to Claude AI where journaled.
- 05a: Expand for selected media, diagnostics, speed test and text input; keep supporting context useful.
- 05b: Release temporary space and restore prior composition when the task ends.
- 05c: Freeze geometry around active touches/typing; routine telemetry must not move windows.
- 05d: Alerts remain visible without stealing focus or destroying the current task.
Gate: actions produce useful space automatically with no repeated oscillation or unnecessary manual arrangement.

### Step 06: Finalize coordinated motion and hit testing
Status: FINALIZED. Dependencies: 04, 05.
Attribution: Codex baseline; later corrections credited to Claude AI where journaled.
- 06a: Animate bounds, content transitions and emblem together; prototype 200–350 ms timing, then measure.
- 06b: Support interruption/reversal from current geometry; clip all moving content correctly.
- 06c: Keep touch targets aligned; hidden systems receive no input.
- 06d: Add reduced motion; preserve state during rapid focus/minimize/return sequences.
Gate: no stale frames, detached controls, input leaks or lost state during stress sequences.

### Step 07: Build and apply the unified LCARS visual system
Status: IMPLEMENTED. Dependencies: 04, 06.
Attribution: Codex Step 07a foundation; Claude AI continuation, per user attribution.
- 07a: Consolidate palette, fonts, type hierarchy, rails, curved elbows, caps, segments, icons and interaction states.
- 07b: Replace conflicting accumulated CSS and generic closed boxes with coherent open-sided structures.
- 07c: Apply styling to home, focused systems, menus, dialogs, lists and error states.
- 07d: Keep thin visible elements backed by generous touch targets; distinguish decoration from controls.
Gate: screenshot comparison shows one consistent reference-led language across all views, with readable text.

### Step 08: Finalize adaptive emblem composition
Status: VERIFIED. Dependencies: 03, 06, 07.
Attribution: Claude AI, per user attribution; journal reports implementation/verification, not new validation by Codex.
- 08a: Prefer central exposure; allow decorative overlap behind opaque panels without covering important content.
- 08b: Resize/reposition smoothly; switch stacked/horizontal identity composition as needed.
- 08c: Avoid oversized empty reservations and jumping between similar gaps; retain small header identity in focus.
Gate: emblem remains integrated through open/minimize/preview/reflow scenarios without hiding controls or text.

### Step 09: Complete Files and media as the first adaptive workflow
Status: VERIFIED. Dependencies: 04–08.
Attribution: Claude AI, per user attribution; journal reports implementation/verification, not new validation by Codex.
- 09a: Responsive list, single-tap folder navigation, selection, path/search and custom LCARS type icons.
- 09b: Stable C:/D:/E: mapping, mounted-volume updates and missing-drive recovery.
- 09c: Image side preview plus properties; stack when narrow; allocate space automatically; one-tap larger viewer.
- 09d: Video preview, play/pause, seek, duration, expansion and playback-state preservation.
- 09e: Open and recoverable Trash with feedback; loading, errors, unsupported media and permissions.
Gate: rapid file selection shows the correct preview; focus/return and reflow preserve context; all actions work with touch.

### Step 10: Complete System and Diagnostics
Status: VERIFIED. Dependencies: 04–08; use patterns proven in 09.
Attribution: Claude AI, per user attribution; journal reports implementation/verification, not new validation by Codex.
- 10a: Responsive useful CPU/memory/storage/uptime/history instrumentation based on actual guest hardware.
- 10b: Structured process list, sorting/filtering where useful and readable diagnostic findings.
- 10c: Compact summaries grow into details; show unavailable sensors honestly.
- 10d: Clear running/cancelled/completed/failed states; separate inspection from disruptive actions.
Gate: useful inspection without terminal-style dumps; actual data, readable actions and retained diagnostic context.

### Step 11: Complete Network and alert workflows
Status: VERIFIED. Dependencies: 04–08; use patterns proven in 09.
Attribution: Claude AI, per user attribution; journal reports implementation/verification, not new validation by Codex.
- 11a: Persistent internet/local-only/offline/limited/captive state where detectable.
- 11b: Transition-driven visual/audio alerts, no duplicate alarm spam, clear recovery and sound preference.
- 11c: Responsive speed-test progress/results/cancel/failure with transfer disclosure.
- 11d: Structured adapter/route/DNS/reachability diagnostics and explicit supported repair actions with results.
Gate: real disconnect/reconnect, failure and cancellation tests pass without misleading status or lost browsing context.

### Step 12: Finish Start, taskbar, Preferences and workspace modes
Status: VERIFIED. Dependencies: 07, 09–11.
Attribution: Claude AI, per user attribution; journal reports implementation/verification, not new validation by Codex.
- 12a: Compact header, identity and rightmost Exit; no permanent left navigation rail.
- 12b: Start search and all-system access; active/open/minimized taskbar states and touch overflow.
- 12c: Keep fullscreen and infrequent controls in settings/menu; Exit distinct from OS power actions.
- 12d: Preserve saved Desktop/Console toggle and consistent preferences; desktop option is a desktop.
- 12e: Finish calculator and remaining Utilities controls using shared responsive patterns.
Gate: every system can be reached/restored with touch and shell controls remain compact at supported widths.

### Step 13: Finalize optional manual arrangement
Status: VERIFIED. Dependencies: 03, 05, 06, 12.
Attribution: Claude AI, per user attribution; journal reports implementation/verification, not new validation by Codex.
- 13a: Enter Basic first; drag/drop swap plus accessible tap alternative.
- 13b: Basic dropdown offers Advanced; Advanced unlocks move/resize and precise touch alternatives.
- 13c: Done, Cancel and Auto Fit; geometry locked outside Arrange.
- 13d: Persist manual preference; temporary attention returns to it; recover gracefully after viewport changes.
Gate: manual override works without becoming a requirement for routine tasks or fighting automatic layout.

### Step 14: Finalize whole-app touch and accessibility
Status: IMPLEMENTED. Dependencies: 02, 09–13.
Attribution: Claude AI, per user attribution; journal reports implementation/verification, not new validation by Codex.
- 14a: Verify tap/scroll/drag separation on actual hardware; keyboard/mouse checks supplement, not replace, touch.
- 14b: Keyboard-aware layout, text-entry access and focus restoration throughout.
- 14c: Accessible names/roles, focus order, pressed feedback, contrast and reduced motion.
- 14d: Eliminate hover-only, double-tap and right-click dependencies.
Gate: all primary journeys complete with touch only; hardware-unverified items stay open.

### Step 15: Complete native Fedora application integration
Status: VERIFIED. Dependencies: early boundary proof in 02; 12, 14.
Attribution: Claude AI, per user attribution; journal reports implementation/verification, not new validation by Codex.
- 15a: Reliable discovery/search/launch and useful LCARS icons; progress/failure handling.
- 15b: Implement the supported running-app/return-to-console and external-window strategy from 02.
- 15c: Verify spatial integration only where actually supported; explicitly track any unsatisfied external-window requirement.
Gate: native apps remain reachable and return paths work; built-in versus external capabilities are accurately documented.

### Step 16: Complete persistence and failure recovery
Status: IMPLEMENTED. Dependencies: 09–15 (basic state preservation already required in 04).
Attribution: Claude AI, per user attribution; journal reports implementation/verification, not new validation by Codex.
- 16a: Versioned durable settings/layouts with atomic writes and invalid-data recovery.
- 16b: Removed drives/files, missing codecs, unavailable telemetry, launch failures and permission errors.
- 16c: Restart during activity; do not silently restart costly tests or unexpected playback.
Gate: each failure has a usable recovery path without a terminal; durable state survives relaunch.

### Step 17: Resolve rendering/cursor reliability and tune performance
Status: BLOCKED. Dependencies: 06, 14–16; fix blockers earlier whenever encountered.
Attribution: Claude AI investigation, per user attribution; hardware verification remains blocked.
- 17a: Reproduce original pointer disappearance/corruption across app/UTM/macOS transitions.
- 17b: Test menus, media, fullscreen, display changes and suspend/resume.
- 17c: Measure animation/input/resource behavior in Fedora UTM; tune without breaking touch targets or fidelity.
Gate: repeatable test matrix passes; renderer workaround alone is not proof; record environment and remaining limitations.

### Step 18: Finish installation and optional desktop/session use
Status: IMPLEMENTED. Dependencies: 02, 15–17.
Attribution: Claude AI, per user attribution; journal reports implementation/verification, not new validation by Codex.
- 18a: Repeatable install/launch/update path; optional startup/fullscreen configuration.
- 18b: Recovery and normal Fedora desktop access; distinguish Exit, logout and power actions.
- 18c: Multi-display/scaling, suspend/resume and supported session behavior.
Gate: LCARS is usable as a primary interface without trapping the user or removing recovery access.

### Step 19: Finalize visual and motion polish
Status: NOT STARTED. Dependencies: 08–18.
Attribution: Pending; no completed implementation asserted for either assistant.
- 19a: Compare home/focused/media/diagnostic/menu states with the reference at actual viewport sizes.
- 19b: Tune density, curved rails, typography, icons, motion and configurable sound.
- 19c: Remove generic leftovers, misleading decorative controls, flashes and awkward empty regions.
Gate: consistent finished appearance while idle and in motion; no regression in readability or operation.

### Step 20: Run final acceptance and close the project
Status: NOT STARTED. Dependencies: 01–19 FINALIZED or explicitly user-approved scope revisions.
Attribution: Pending; no completed implementation asserted for either assistant.
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

2026-10-08 / Step 04 CURRENTLY WORKING ON: access recovered after UTM input interruption. Guest audit shows only our console/ui/tools changes; LCARS process still running (was hidden, not proven crashed). Initial adapter build/Clippy/32 tests passed; four/six-panel UI observed. Installing staged accessible summaries and Network reflow, then rechecking. Remaining: narrow/focused/media preservation and final acceptance.

2026-10-08 / Step 04 CURRENTLY WORKING ON: access recovered after UTM input interruption. Guest audit shows only our console/ui/tools changes; LCARS process still running (was hidden, not proven crashed). Initial adapter build/Clippy/32 tests passed; four/six-panel UI observed. Installing staged accessible summaries and Network reflow, then rechecking. Remaining: narrow/focused/media preservation and final acceptance. Added explicit GTK integration check for live resize/input retention and shared prepared media seek/pause; run with the existing test fixture.

2026-10-08 / Step 04 CURRENTLY WORKING ON: access recovered after UTM input interruption. Guest audit shows only our console/ui/tools changes; LCARS process still running (was hidden, not proven crashed). Initial adapter build/Clippy/32 tests passed; four/six-panel UI observed. Installing staged accessible summaries and Network reflow, then rechecking. Remaining: narrow/focused/media preservation and final acceptance. Added explicit GTK integration check for live resize/input retention and shared prepared media seek/pause; run with the existing test fixture.

2026-10-08 / Step 04d CURRENTLY WORKING ON: resumed and audited guest changes. 32 unit tests and explicit GTK input/media/page checks passed. Found 645px actual allocation for 640px requested; fixing shell minimum and adding exact allocation checks. Removed accidental terminal text inserted at tracker start during focus recovery; remaining tracker retained.

Step 04 verification: shell now fits 640px exactly. The 900px portrait-height check was constrained to 777px by the VM desktop work area; changing portrait check to 740px within the real display. No app workaround for a window-manager limit.

2026-10-08 / Step 04 checkpoint VERIFIED in part: fixed taskbar minimum width using a horizontally scrollable shortcut strip, keeping Start/status/clock outside it. All six pages render at exact requested 1000x720, 640x540, 800x440, 640x740, then 1000x720 dimensions. Live GTK check also retains entry values and prepared paused video seek position. Build, strict Clippy, 32 unit tests plus one explicitly run GTK integration test passed. Evidence: /mnt/macos/responsive04/verified.log. Step 04 remains CURRENTLY WORKING ON for full workflow preservation and visual acceptance; Step 05 not started.

2026-10-08 / Step 04c CURRENTLY WORKING ON: full Console/ Desktop transition regression using actual Files and Utilities widgets; verify selection/path/filter/scroll and read-only diagnostic job completion while hidden. Fix any state losses before Step 05. Scope src/ui.rs, src/console.rs, docs/responsive-adapters.md.

Step 04c: real Files and diagnostic continuity passed. Fixing transient negative width before allocation, and retaining/restoring last content input on focus/home and mode transitions; adding focus regression.

2026-10-08 / Step 04 FINALIZED: real Files path/filter/selection/metadata/scroll retained across focus/home and Desktop/Console transitions; background read-only diagnostics completes and survives mode changes. Added text-input memory/restore after transitions and cleared focus before reparenting; fixed pre-allocation negative width. GTK integration passes without warnings/criticals. User screenshot target and full final journey matrix remain future acceptance, not claimed complete. Step 05a CURRENTLY WORKING ON as above.

2026-10-08 / Step 05a VERIFIED: explicit preview/transfer/diagnostic demand wired to automatic solver; most-recent active task, no telemetry trigger, no focus stealing. Baseline restoration and real GTK enlargement/restoration passed; 34 unit tests, strict Clippy/build and GTK check pass. Evidence /mnt/macos/attention05/build.log. Step 05b–05d CURRENTLY WORKING ON: restore manual composition, multi-contact input freeze and queued updates, alert noninterference; then Step 06 reduced motion and coordinated transitions. Scope console.rs, attention.rs, ui.rs tests and docs.

2026-10-08 / Step 05 FINALIZED: explicit demand; temporary enlargement in automatic and manual layouts; baseline restored on completion/Arrange; saved geometry excludes temporary emphasis; minimized systems stay hidden. Distinct touch sequences and pointer holds defer automatic geometry; text focus defers attention; manual Arrange drag stays enabled. Latest eligible demand applies after release. Alerts reviewed: update banner/notifications without focus/attention requests; actual network transition tests remain 11. 35 unit tests and GTK automatic/manual restoration + focus/Files/diagnostic continuity pass. Step 06 CURRENTLY WORKING ON.

2026-10-08 / Step 06 IMPLEMENTED; automated verification passed: 36 unit tests, strict Clippy/build and GTK rapid reversal/reduced-motion/hidden-target assertions, plus previous workflow/attention matrix. Evidence motion06/build.log. CURRENTLY WORKING ON visual launch/focus/home inspection before finalization.

Step 06 visual review: home and focused Files render correctly in fresh guest build; pointer coordinates in UTM can lag, so keyboard navigation supplements visual checks (hardware/input audit remains 17). Adding the missing visible reduced-motion preference before closing this step; existing saved choice retained.

2026-10-08 / Step 06 FINALIZED: panel bounds/opacity and emblem bounds/opacity share 300 ms eased timing; reverse from displayed geometry, pause/rebase for contacts, clip content and disable moving/hidden input. Reduced motion is instant and exposed in Preferences with persistence test. GTK stress switches every 60 ms and verifies final Files state/hidden-target exclusion; all prior checks pass. Home/focused Files inspected in fresh app. Step 07 CURRENTLY WORKING ON.

2026-10-09 / Step 07a–07c CURRENTLY WORKING ON: resume staged stylesheet consolidation and Preferences cleanup; verify exact guest baselines before installation, then CSS parsing and viewport/workflow regression.

Step 07a scope refinement CURRENTLY WORKING ON: CSS and Cairo instruments need the same palette source. Adding src/theme.rs and integrating main/ui/overview; nine shared palette tokens, no dependency changes. Consolidated CSS parses and all viewport/workflow tests passed before this refinement.

2026-10-09 / Step 07b CURRENTLY WORKING ON: in-guest screenshot capture established (ImageMagick import with -screen on display :1; GNOME D-Bus Screenshot returns AccessDenied and plain root-window capture is black under the compositor). First visual evidence captured and exported to the Mac. Class audit found no selectors referenced without a definition and five dead rules (band-amber, band-coral, band-lilac, header-end-cap, system-id). Replacing the closed-box Cairo instruments in src/overview.rs with open-sided segmented LCARS forms and dropping the dead rules. Scope: src/overview.rs, assets/lcars.css. Verification: cargo fmt, build, clippy -D warnings, tests, GTK adapter test, then screenshot comparison against preview-fixtures/reference.jpg.

2026-10-09 / Step 07b VERIFIED: closed-box Cairo instruments replaced with open-sided segmented forms. CPU/RAM/HOME gauges now use a lilac spine and foot with discrete lit segments over a RULE-toned unlit scale, no enclosing outline; the history graph lost its full grid rectangle for a left axis rail, baseline and two interior rules. Removed five dead selectors (band-amber, band-coral, band-lilac, header-end-cap, system-id); assets/lcars.css 8289 -> 8052 bytes. cargo fmt, build, clippy -D warnings, 18 unit tests and the GTK adapter integration test all pass with the media fixture. Visual evidence captured in-guest for the first time: /mnt/macos/shots07/t-winid.png (before) and after-07b.png, start-menu.png. 07c partially evidenced: Start menu, search, application list and system buttons already carry the consolidated language. Remaining in Step 07: 07c dialogs and error states, 07d touch-target audit, then the screenshot gate against preview-fixtures/reference.jpg.

2026-10-10 / Step 07a-07b CURRENTLY WORKING ON: user directs the visual system to match preview-fixtures/reference.jpg as faithfully as possible. Sampled the reference by colour histogram: orange #ff9900, salmon #cd6667, periwinkle #9c99ff, pink #cb99ce. The shipped palette had drifted pastel (amber #ffcb64). Retargeting src/theme.rs to the sampled values and rebuilding the panel chrome as true LCARS elbows: same-colour perpendicular arms joined by a large radius, replacing the mixed-colour thin borders. Scope: src/theme.rs, assets/lcars.css. Verification: fmt/build/clippy/tests plus screenshot comparison.

2026-10-10 / Step 07 IMPLEMENTED, awaiting the user's visual sign-off at the gate. Palette retargeted to values sampled from preview-fixtures/reference.jpg by colour histogram: AMBER ff9900, CORAL cd6667, LILAC 9c99ff, ROSE cb99ce, MUTED 9999cc, PRESS ffcc66. Panel chrome rebuilt as true elbows: console-region now an 18px rose rail with a 42px top-left radius, band title flush against it; card, history-panel and telemetry-panel use same-colour perpendicular arms with 26-30px radii instead of mixed-colour thin borders. 07b open-sided instruments and dead-rule removal verified earlier today. 07c evidenced for home, focused systems, Start menu, search, application and file lists; the Trash confirmation dialog inherits the consolidated dialog/button styling but has not been visually captured, because doing so means trashing a real file in the user's VM. 07d satisfied: every interactive control requests 44-48px. Verification: cargo fmt, build, clippy -D warnings, 18 unit tests and the GTK adapter integration test all pass. Evidence /mnt/macos/shots07/after-palette.png. Gate not self-certified: screenshot comparison is the user's call. Step 08a CURRENTLY WORKING ON: emblem was observed clipped by the Files panel at 1464px width in t-winid.png, which 08a forbids.

2026-10-10 / Step 07 palette consolidation completed and Step 08a VERIFIED. Found the emblem still carried hardcoded RGB approximations of the pre-07a palette in ui.rs draw_emblem and the console.rs identity text; both now resolve through crate::theme, so nothing outside theme.rs defines a colour. Fixed a real 08a defect: the identity text was centred at a fixed offset with no width constraint, so SYSTEM AVAILABLE measured about 215 units inside the 210-unit vertical design space and was clipped by the Files panel at wide viewports. It now shrinks to 2*min(center, design_w-center)-8 before drawing. Verified at 1464x777, the width that reproduced the clipping; evidence /mnt/macos/shots07/emblem-fixed.png. fmt, build, clippy -D warnings, 18 unit tests and the GTK adapter test pass. Remaining in 08: 08b stacked/horizontal identity switching and 08c reservation stability across open/minimize/preview/reflow.

2026-10-10 / Step 07b CURRENTLY WORKING ON, two user-reported defects. First, the panel outline read as two separate lines because the vertical rail was rose and the horizontal title arm coral; pixel sampling confirmed they are flush at x=22/24 with no gap, so the break was a colour discontinuity, not geometry. An LCARS elbow is one colour bent through a radius, so the rail now matches the title arm. Second, panel content was clipped on the right by exactly the rail width: tiles are sized to w but the CSS border reduces the content box, and the overview was still requesting w, so the minimize control was cut (content ended at x=1443 of 1464). Rail width now lives in theme::RAIL and is emitted into the stylesheet, so the layout maths and the CSS cannot drift apart. Scope: src/theme.rs, src/console.rs, assets/lcars.css.

2026-10-10 / Step 07 VERIFIED, both user-reported defects fixed and confirmed by pixel measurement. Elbow continuity: the title row now samples one colour cd6667 from x=8 through x=60, so rail and arm are a single bent shape rather than two lines. Clipping: panel content now reaches x=1455 of a 1464px window instead of stopping at 1443, and the minimize control is whole on every panel; the residual 9px is the intended inter-panel gap from the -6.0 width term plus console-surface padding, not a cut. theme::RAIL owns the rail width and emits its own CSS rule, so layout maths and stylesheet cannot drift. fmt, build, clippy -D warnings, 18 unit tests and the GTK adapter test pass. Evidence /mnt/macos/shots07/elbow-fixed.png. Note for future capture rounds: xdotool search --class lcars also matches the app's tooltip window, which produced a 200x20 capture of a tooltip and misleading colour samples; select the largest-area match instead. GNOME idle-blanked and locked the session mid-round, which produced an all-black desktop capture; idle-delay and screensaver lock are now disabled for this session.

2026-10-10 / Step 07 elbow alignment corrected on user report. The arm was not flush with the rail because border-top-left-radius was 42px against an 18px border, giving an inner radius of 24px that clipped a diagonal notch out of the title arm's top-left. The emitted radius now tracks the rail as RAIL+2, so the inner corner is effectively square and the arm meets the rail exactly. Rail slimmed from 18px to 10px at the user's request; because theme::RAIL drives both the stylesheet and the layout maths, the content box followed automatically with no new clipping. Evidence /mnt/macos/shots07/rail-slim.png.

2026-10-10 / Step 07 header alignment corrected on user report. The title arm sat low because console-band-title and window-glyph were min-height 24px vertically centred inside a 48px band, leaving ~12px of background above and below, so the arm never met the panel top. Both now fill the 48px band, making the header flush with the top of the rail and completing the elbow. Evidence /mnt/macos/shots07/header-flush.png. Step 07 visual work complete pending the user's gate sign-off.

2026-10-10 / Step 08c CURRENTLY WORKING ON: logo_region scores every maximal empty rectangle by area weighted toward centre and takes the strict maximum, with no preference for the gap the identity already occupies. Two gaps of near-equal score therefore swap the winner on any telemetry-driven bounds change, which is the jumping 08c forbids. Adding hysteresis: the incumbent is retained unless a rival beats it by a clear margin. Scope: src/console.rs.

2026-10-10 / Step 07 header and controls refined on user report, Step 08c IMPLEMENTED.
Header: filling the whole 48px band read as too heavy, so the visible arm is now 30px anchored to the top of its 48px touch target. This is exactly 07d's intent, a thin visible element backed by a generous target, and it stays flush with the rail.
Window controls: compared against the reference crop, which shows colour-blocked segments whose terminating segment carries a full rounded end cap, with small marks rather than large glyphs. Minimize is now amber with a 15px right cap instead of coral with 13px, and glyphs dropped from 24px to 15px. Glyphs were kept rather than reduced to the reference's bare dot, because they are the only affordance distinguishing the two controls and 07d requires decoration to stay distinguishable from controls.
08c: logo_region now takes the incumbent region and keeps it unless a rival scores 15% better, with region_score and region_clear factored out. The identity can no longer swap between near-equal gaps on routine telemetry updates. Evidence /mnt/macos/shots07/buttons.png.

2026-10-12 / Session resumed after a host kernel panic (Apple DCP display assert) hard-reset the Mac while the VM was live, leaving Fedora to boot into emergency mode. Project verified intact: HEAD 460218f, all Step 07/08 edits present, 9p share remounted, build and 18 tests pass.
Capture pipeline rebuilt to be independent of the user's desktop: the graphical session is at the GDM greeter and no X display was reachable, so verification now runs the app under Xvfb on :99 via dbus-run-session. Renders identically to the real session. Two gotchas recorded: xdotool --onlyvisible finds nothing without a window manager, and the app owns several windows, so select the largest whose name starts with LCARS rather than matching by class.
Step 07 per-panel accents IMPLEMENTED at user request to match the reference: theme::ACCENTS maps one colour per system in panel order (amber, coral, lilac, rose, tan, mauve), emitted as .accent-N rules binding the rail and title arm, applied in console.rs by tile index.
Font retarget ATTEMPTED AND REVERTED. Oswald and Roboto Condensed both render a .notdef box on every Applications row; Nimbus Sans Narrow does not. Application display names are clean ASCII when read from Gio.AppInfo, and the Files list is unaffected, so the missing glyph is not in the name itself and is not yet isolated. Cairo's toy font API does no fallback, which is why this surfaces in the instruments but not in GTK/Pango text. FONT is back to Nimbus Sans Narrow pending a proper fix; theme::FONT now owns the family for both the stylesheet and the Cairo instruments, so the eventual switch is a one-line change.

2026-10-12 / Step 08 VERIFIED and a Step 03e defect found and fixed.
08b: drove the window across 1600x560, 900x900, 760x500 and 1464x777 on the virtual display. The identity switches composition correctly, horizontal (emblem left, text right) when the gap is wide and stacked when it is tall or narrow, and reflows without stale frames. Evidence /mnt/macos/shots07/c08-wide.png and c08-small.png.
03e defect: layout::solve computes an overflow flag when the packed canvas exceeds the viewport, but nothing consumed it and the shell is a plain Overlay, not scrollable. The console passed the solver's canvas straight to set_size_request, so at constrained viewports whole panels were hidden under the taskbar: at 760x500 panels 03 and 04 were sliced mid-content and 05 and 06 were not reachable at all, and at 1600x560 panel 06 and the emblem were cut. Step 03 had been marked FINALIZED without its overflow fallback ever being implemented. The canvas is now clamped to the viewport so tiles shrink instead of disappearing; all six panels and the emblem are fully visible at both failing sizes. fmt, build, clippy -D warnings, 18 unit tests and the GTK adapter test pass.
Step 09a CURRENTLY WORKING ON: auditing the existing Files workflow against 09a-09e before changing anything.

2026-10-12 / Step 09 VERIFIED by driving the real workflow on the virtual display rather than by reading code. Created /home/fedora/lcars-test holding a PNG, a WebM, a text file and a deliberately truncated PNG, then navigated and selected each.
09a: path entry, HOME/UP/GO, live filter, single-tap navigation and distinct LCARS type icons for folders versus files all work; navigating to the test folder updated the listing and the entry count.
09b: drive aliases render as C: SYSTEM /, DOCUMENTS, DOWNLOADS, EXCHANGE and D: share, backed by the existing assign_drive_labels unit test for reordering and unplugging.
09c: image preview renders with full properties, size, permissions, mtime and path.
09d: video preview reports VIDEO / 320 x 180 / 3.0 s with a play control, and the preview pane auto-expands via the Step 05 attention path when media is selected.
09e: the truncated PNG produces 'Preview unavailable: Unsupported image format' with properties still shown, so a bad file degrades instead of failing.
04d defect found and fixed: the detail column was a plain box whose natural height exceeded the viewport, so OPEN and MOVE TO TRASH fell below the fold and were unreachable at 1464x777. Inspection now scrolls inside the column while both actions stay pinned, which is what 04d requires. A first attempt used propagate_natural_width and let the column eat the list's width; corrected with hexpand false.
Unverified and still open: physical touch confirmation of these journeys, which belongs to Step 14 and cannot be settled on a virtual display.
Step 10 NOT STARTED next: System and Diagnostics, reusing the patterns proven here.

Step 10 CURRENTLY WORKING ON: auditing System and Diagnostics against 10a-10d by driving the focused view before changing anything. Scope read-only until the audit lands.

2026-10-12 / Step 10 VERIFIED. The audit found 10a and 10c already satisfied: the focused System view carries live processor load, memory, network receive, uptime, a CPU history graph and storage, and states honestly that readings reflect Fedora rather than the Mac host and that sensors UTM does not expose are unavailable.
10b was the real gap and directly violated the step gate. SCAN SYSTEM wrote raw uname, df and ps output into a monospace TextView, which is exactly the terminal-style dump the gate forbids. Added services::parse_storage and services::parse_processes with StorageRow and ProcessRow types and two unit tests covering malformed lines, taking the suite from 18 to 20. tools::render_findings now builds readable sections: a KERNEL card, STORAGE / MOUNTED VOLUMES with mount, type, size and free plus a usage bar and percentage per row, and TOP PROCESSES / BY PROCESSOR SHARE as a headed PID/COMMAND/CPU/MEMORY table sorted by processor share. The TextBuffer is retained as the machine-readable record the overview summary and the GTK integration test read, so that contract is unchanged.
10d: the scan now reports explicit states, SCANNING while it runs, SCAN COMPLETE with volume and process counts and the reassurance that nothing was terminated, and SCAN FAILED with the reason if any command errors. Inspection stays separated from disruptive actions; no process can be killed from this view.
Evidence /mnt/macos/shots07/k10-structured.png and l10-processes.png. fmt, build, clippy -D warnings, 20 unit tests and the GTK adapter test pass.
Step 11 NOT STARTED next: Network and alert workflows.

Step 11 CURRENTLY WORKING ON: auditing network state detection and alert behaviour before any change. Real disconnect testing must not sever the SSH session used to drive the work, so connectivity will be degraded selectively rather than by taking the interface down.

2026-10-12 / Step 11 VERIFIED, including the gate's real disconnect and reconnect test.
Method: rather than take the interface down, which would sever the SSH session driving the work, NetworkManager's connectivity probe was pointed at an unreachable URI with a detached 90 second auto-restore as a safety net. NM moved to connected (site only):limited and back to connected:full on restore.
11a VERIFIED: the limited state propagated to all three surfaces, the panel heading switched ONLINE to LIMITED, the taskbar to NETWORK LIMITED.
11b VERIFIED: a COMMUNICATIONS ALERT / LIMITED banner appeared on transition and cleared on recovery. Alerts are driven by state change, not by polling, so there is no duplicate alarm spam, and the Files listing kept its path and selection throughout, satisfying the gate's requirement that browsing context is not lost.
11c VERIFIED: progress, final results and the transfer disclosure all render. Found and fixed a clipping defect: the diagnostics scroller is vexpand with a 230px floor, which pushed the speed-test result under the panel edge so only the latency line was visible and the download and upload figures were lost. Floor reduced to 140px; all four result lines now show. Cancellation could not be exercised through the interface because the test completes in about a second on this link, so the UI cancel path remains unverified by observation; it is covered by the existing network::tests::cancelled_test_does_not_start_network_io unit test, and a guard was added so a cancelled run reports TEST CANCELLED rather than leaving a partial latency reading that would read as a completed measurement.
11d VERIFIED: the adapter report was a monospace nmcli dump, the same violation found in 10b and contrary to 11d's requirement for structured findings. Added services::parse_devices and services::parse_addresses with NetDevice and two unit tests, taking the suite to 22, and render_network_findings now shows an ADAPTERS table with device, type, state and connection, an ADDRESSES table, and ROUTES and NAME RESOLUTION sections.
Evidence n11-degraded.png, o11-restored.png, r11-structured.png, s11-scrolled.png, x11-result.png.
Step 12 NOT STARTED next: Start, taskbar, Preferences and workspace modes.

Step 12 CURRENTLY WORKING ON: auditing Start, taskbar, Preferences and workspace modes by driving minimize and restore, Start search, the Desktop/Console toggle and the calculator. Read-only until the audit lands.

2026-10-12 / Step 12 VERIFIED by driving every element rather than reading code.
12a: the masthead is a thin identity band with Exit at the far right and there is no permanent left navigation rail.
12b: minimising Files reflowed the remaining five panels and the emblem claimed the freed gap; restoring from the taskbar brought it back focused. Start search filters live, typing calc narrowed to Calculator and LibreOffice Calc, and all six systems plus HOME, ARRANGE and AUTO FIT are reachable from the menu.
12c: FULLSCREEN sits in Preferences rather than permanent chrome, and the help text states F11 toggles fullscreen, Escape dismisses Start first and Exit returns to Fedora, keeping Exit distinct from OS power actions.
12d: toggling CONSOLE MODE off switched the shell to Desktop mode and the choice survived a full restart, then toggled back to Console. The desktop option really is a desktop, a single overview surface rather than the six-panel console.
12e: the calculator computes correctly, 12 x 7 returned 84, and division by zero reports Cannot divide by zero instead of an infinity or a crash.
Evidence y12-minimized.png, z12-search.png, c12-mult.png, c12-divzero.png, d12-prefs.png, e12-desktopmode.png.
Step 13 NOT STARTED next: optional manual arrangement.

Step 13 CURRENTLY WORKING ON: driving Arrange to check Basic entry, the Advanced unlock, Done/Cancel/Auto Fit and geometry locking outside Arrange.

2026-10-12 / Step 13 VERIFIED, with a missing control implemented.
13a: Arrange opens in Basic and prompts DRAG ONTO ANOTHER SYSTEM TO SWAP; the tap alternative works, selecting one band then tapping another performs the swap without any drag.
13b: the mode dropdown offers BASIC, checked by default, and ADVANCED, which reveals the resize handles.
13c had a real gap. The step requires Done, Cancel and Auto Fit, but only DONE and AUTO FIT existed and leaving Arrange always persisted, so an unwanted edit could not be abandoned. Added cancel_arrange: entering Arrange now snapshots regions, hidden flags and the automatic layout setting into arrange_snapshot, and CANCEL restores that snapshot, exits without persisting and reports ARRANGEMENT DISCARDED / HOME. Verified by swapping two systems and cancelling, which restored the original order. Clippy's type_complexity fired on the snapshot tuple, factored into an ArrangeSnapshot alias.
13d: the manual preference persists through DONE, and attention_base already restores the manual composition after temporary attention, exercised earlier in Step 05.
Evidence g13-arrange.png, h13-dropdown.png, j13-crop.png, k13-cancelled.png.
Step 14 CURRENTLY WORKING ON: whole-app touch and accessibility. 14a requires physical touch hardware and cannot be settled on a virtual display, so it stays explicitly open per the step's own gate; 14b, 14c and 14d are auditable here.

2026-10-12 / Step 14 IMPLEMENTED, deliberately not VERIFIED because 14a cannot be settled here.
14a REMAINS OPEN: tap, scroll and drag separation must be confirmed on physical touch hardware. A virtual display driven by synthetic pointer events cannot stand in for that, and the step's own gate says hardware-unverified items stay open. This is the long-standing limitation already recorded in the baseline.
14b: Tab moves focus through the shell and the stylesheet draws a visible amber focus ring via button:focus-visible, entry:focus and searchentry:focus-within. Text entry and focus restoration across transitions were proven earlier by the GTK adapter test, which checks entry values survive wide/narrow/wide reflow.
14c: found and fixed a real defect. The maximize and minimize controls carried only their decorative glyph as an accessible name, so assistive technology would announce a box or a dash. They now publish accessible::Property::Label from the same text as their tooltip. Custom-drawn panels and instruments already publish labels. Reduced motion is implemented and covered by motion_duration_clamps_and_reduced_motion_finishes_immediately.
14d: no hover-only, double-tap or right-click dependency exists. The single motion controller in ui.rs only re-asserts the cursor shape, which is the UTM cursor workaround, not an interaction requirement. Tooltips supplement controls that also carry visible labels or accessible names.
Step 15 CURRENTLY WORKING ON: native Fedora application integration.

2026-10-12 / Step 15 VERIFIED, and the Oswald font blocker from 2026-10-10 is resolved.
15a: discovery, search and launch all work. Typing calculator in Start filtered to the right entries and launching started gnome-calculator as a real process, confirmed by pgrep. Launch failures surface as Launch failed with the error text.
15b: native apps own their own Fedora windows, which is the only strategy Step 02 found supportable, so the integration is now explicit rather than implied. The Applications page records LAST STARTED / <app> and carries a RETURN TO CONSOLE control that presents the console window, so there is a reliable way back that does not depend on the Fedora window switcher. The old copy told the user to use that switcher; it now states plainly that the app opened in its own window.
15c: external spatial integration REMAINS UNSATISFIED and is recorded as such rather than quietly dropped. Step 02 established that this GTK4 binding exposes no Gtk.Socket foreign embedding and that true embedded arbitrary-app surfaces would need a separate compositor or window-management approach. Launching gnome-calculator confirmed the boundary in practice: it composites above the console as an independent window and is not reflowed by the solver. No claim is made that native app internals can be restyled.
FONT RESOLVED: the tofu that forced the revert to Nimbus came from the Applications rows, which prefixed every entry with U+25B0 BLACK PARALLELOGRAM as a decorative marker. Nimbus carries that glyph, Oswald and Roboto Condensed do not, and Cairo's toy font API does no fallback, which is why it appeared in the instruments but not in GTK text. The marker is now drawn as a CSS left border on .app-tile and the label is the plain application name, so no typeface has to carry it. theme::FONT is back to Oswald and the console renders clean. Evidence n15-oswald.png.
Step 16 CURRENTLY WORKING ON: persistence and failure recovery.

2026-10-12 / Step 16 IMPLEMENTED.
16a VERIFIED: settings and layout are versioned (console-layout-v2.json), written atomically to a .tmp file then renamed, and the loader rejects records whose region or hidden counts do not match the panel set. Tested destructively by overwriting the saved layout with malformed JSON; the app started normally on the default layout with no panic in the log.
16b PARTLY VERIFIED: failure paths exist and several were observed directly, a truncated PNG giving Preview unavailable: Unsupported image format, launch failures reporting Launch failed, scan failures reporting SCAN FAILED, and save failures reporting Could not save. load_directory reports Could not read folder with the underlying error and navigate sets the path before loading, so a read failure surfaces rather than being swallowed. The permission-denied directory case could NOT be confirmed through the interface: synthetic keystrokes stopped reaching the path entry in that session, so the listing never changed. This is a limitation of the test harness, not evidence of a defect, and the case stays open for confirmation.
16c VERIFIED by repetition: the app has been restarted many times across this session and always returns to READY / no test has run with no playback resumed, so no costly test or unexpected media restarts on launch.
Also fixed for 07a completeness: the file-type icon in the Files list drew with a hardcoded copy of the pre-07a amber; it now resolves through crate::theme::AMBER.
Step 17 CURRENTLY WORKING ON: rendering and cursor reliability, which is largely hardware-dependent.

2026-10-12 / Step 17 BLOCKED on hardware, stated plainly rather than papered over.
17a and 17b cannot be settled from here. Both are about pointer disappearance and rendering corruption across application, UTM and macOS transitions, and about suspend/resume and display changes. All of this verification work in this session runs on a headless Xvfb display reached over SSH, which by construction has no UTM SPICE display, no virtual GPU and no macOS window server, so it cannot reproduce the fault or prove its absence. The GSK_RENDERER=cairo workaround in main.rs remains in place, and the step's own gate says a renderer workaround alone is not proof.
What permits progress: a session on the real UTM display, with the user present to observe pointer behaviour across focus changes, fullscreen toggles and host sleep/wake.
Relevant new evidence from the host side: the Mac kernel panicked twice while this VM was running, 2026-10-10 and 2026-10-12, both times in AppleDCPDPTXPowerController asserting after powering the DisplayPort rails. UTM, QEMULauncher and QEMUHelper were live in both panic reports. That is the same display-pipeline territory Step 17 covers and may be related, so 17 should be approached with the VM's GPU acceleration disabled rather than with the host left to sleep mid-session.
17c partial: at idle on the virtual display the process sits at 0.0 percent CPU, so nothing is spinning, but animation timing and input latency under the real virtual GPU remain unmeasured.

2026-10-12 / Step 18 IMPLEMENTED.
18a: there was no repeatable install path. The desktop entry pointed at target/debug/lcars-desktop inside the source tree, so the registered application depended on a debug build sitting in a working directory. Added install.sh, which builds an optimised binary, installs it to PREFIX/bin (defaulting to ~/.local), writes the desktop entry to point at the installed path with GSK_RENDERER=cairo, refreshes the desktop database and prints the autostart instruction. Re-running it updates in place, so install and update are the same command. Verified by running it, launching ~/.local/bin/lcars-desktop and capturing the console, which renders identically to the debug build. Release binary is 1.8 MB.
18b: Exit returns to Fedora and is described as such in Preferences, distinct from logout and power actions, and the console is an ordinary application window so the normal Fedora desktop stays reachable. Optional startup is offered rather than imposed, by copying the desktop entry into ~/.config/autostart.
Not claimed: 18 depends on 15 to 17, and 17 is BLOCKED on hardware, so 18 stays IMPLEMENTED rather than FINALIZED.

2026-10-12 / Steps 19 and 20 NOT STARTED and cannot proceed without the user.
Step 19 is visual and motion polish whose acceptance is the user's judgement against preview-fixtures/reference.jpg, and Step 20's gate requires Steps 01 to 19 FINALIZED or explicit user scope revision. Outstanding blockers: 14a physical touch, 17 cursor and rendering on the real UTM display, and the user's visual sign-off on Step 07 and Step 19.

2026-10-22 / Codex documentation task FINALIZED: marked last Codex Step 07a boundary; credited subsequent work to Claude AI per user direction; added mixed-provenance source comments and architecture guide. Reconciled stale checklist statuses 09–12, 14, 16–18 with the existing journal. Preserved unfinished gates and all functional code. Read-only inspection confirmed installed release launcher and NetworkManager event notification flow. No new hardware/runtime acceptance claimed.
