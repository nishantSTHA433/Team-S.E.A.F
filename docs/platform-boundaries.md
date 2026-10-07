# Input and Fedora integration boundary — Step 02

## Measured environment

2026-10-07 read-only probe: Fedora session 2, X11 display :1, GTK 4.10.5. GDK reports pointer and keyboard capabilities only. QEMU USB Tablet and spice vdagent tablet are exposed as MOUSE, not touchscreen. There is no hardware touch proof in this VM. GNOME screen-keyboard-enabled is false; no setting was changed. GTK IMMulticontext and Accessible APIs are available.

The first probe incorrectly assumed :0 and obtained no display. The corrected probe inherits DISPLAY and XAUTHORITY from the terminal session and initializes GTK; only probe-fixed.log's successful :1 result is evidence. Keep tools/platform_probe.py reusable without hard-coded session credentials.

## Interaction contract for adapters

Title/taskbar activation restores and focuses a system; repeating activation returns home. Maximize explicitly focuses, minus minimizes. Content controls consume their own events: file rows select/preview, folders navigate on one tap, Open explicitly launches, video controls act directly. A background activation cover must not cover real controls after Step 04 migration. Capture-phase swipe and arrangement gestures must claim a sequence only after distinguishing scroll from tap; never reinterpret completed scroll as activation. Hidden systems are non-targetable. Basic swap/Advanced geometry editing are restricted to Arrange. Keyboard fallbacks supplement touch, never substitute for hardware sign-off.

Current console uses capture-phase GestureDrag and disables page targeting in home mode. This is why interactive home content requires adapter work rather than only larger text. Model contact tokens permit multi-contact suppression of geometry changes; connect cancellation as well as release events to avoid frozen layouts. GTK text input must expose IM context and keyboard-aware allocation; enabling a GNOME setting alone does not prove the final text-entry flow.

## External application feasibility prototype

The capability probe finds no Gtk.Socket foreign embedding API in this installed GTK4 binding. X11 EWMH advertises _NET_CLIENT_LIST, _NET_ACTIVE_WINDOW and _NET_MOVERESIZE_WINDOW; xprop is installed, wmctrl is not. Thus a later X11 adapter can investigate discovering/activating/moving normal external windows through the window manager. This is capability discovery, not a working moving-window implementation and not proof of internal content reflow. No external window was reparented, manipulated or captured by the probe.

Decision: the six built-in LCARS systems are the supported target for content-aware panel composition and responsive internals. Native Fedora apps continue to launch normally while Step 15 implements explicit launch/running/return integration. Keep external spatial integration OPEN in Step 15: true embedded arbitrary-app surfaces would require a separate compositor/window-management approach, not replacing an external app's internal GTK layout. No silent scope reduction or claim that native app internals can be restyled automatically. If later session/backend differs, re-probe rather than assuming these X11 facilities exist.

## Desktop/session boundary

LCARS remains a normal GTK application under the existing session. Optional primary-desktop startup, recovery, multi-display and session integration are Step 18. Do not replace the working session or disable desktop security to demonstrate appearance. Cairo fallback remains in launch.sh/main.rs; cursor reliability still requires Step 17's sequence tests.

## Gates and follow-up

Step 02 establishes implementation boundaries, not final touch/session certification. Step 03 may build a backend-independent solver using the measured logical viewport interface. Physical touch validation stays in Step 14, external-window implementation in Step 15, cursor performance in Step 17, and desktop/session deployment in Step 18. None of those are marked complete by this probe.
