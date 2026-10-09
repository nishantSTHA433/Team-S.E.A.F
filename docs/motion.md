# Coordinated motion baseline

Panel geometry, opacity and background emblem geometry/opacity share a 300 ms eased transition. New requests start from the last displayed state. Contact holds pause motion; elapsed hold time is removed when motion resumes. Moving panels and hidden systems cannot receive content input. Taskbar navigation remains available for reversals. Manual arrangement can still track a drag.

The existing reduced-motion preference now bypasses animation and is visibly editable in Preferences. Saved choices are preserved. The live GTK regression switches systems every 60 ms, then verifies that only the final focused panel is targetable and that Files content survives. It separately asserts immediate interaction in reduced-motion mode and preference persistence. Evidence: /mnt/macos/motion06/settings.log.

This establishes the guest transition baseline; physical touch, frame-budget measurements on target hardware, cursor re-entry and final visual polish remain tracked separately.
