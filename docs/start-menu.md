# LCARS taskbar and Start menu

The left navigation rail is removed in both Console and Desktop modes. The content now fills the available width. A persistent bottom taskbar provides LCARS / START, System, Files, Network, and Tools controls, plus network status and the live clock.

Start opens an upward LCARS popover with coral and lavender bands, an asymmetric curved edge, and 48-pixel application and system controls. Installed Fedora applications are discovered through GIO and filtered by name. Selecting an application launches it through GIO; launch failures stay visible in the menu. All six console systems remain available beside the application list. Selecting a system closes the menu and uses the current layout mode's navigation. Escape dismisses the menu before leaving fullscreen.

Validation in Fedora 38: cargo build and strict Clippy passed. The sidebar-free overview and Start menu fit the VM display. Application search found Calculator and launched it successfully. Preferences navigation closed the menu and focused the correct panel. Switching between Desktop and Console retained the taskbar; Console overview was restored afterward.

The taskbar currently provides launch/navigation shortcuts, not a list of running native windows. Native applications retain their Fedora windows. Dedicated LCARS application panels, smoother zoom transitions, and additional reference-image refinements remain future work.
