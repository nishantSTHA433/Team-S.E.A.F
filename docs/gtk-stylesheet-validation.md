# GTK stylesheet validation and Git preparation

The application uses GTK 4 CSS, not browser CSS. GTK node selectors such as `window`, `headerbar`, `entry`, `trough`, `:selected`, and named colours such as `@lcars_amber` are intentional. RustRover's web-CSS inspection treated these as errors during commit checks. Changing them to HTML selectors would damage the theme.

The source is now `assets/lcars.gtkcss`: an explicit dialect-specific text asset, embedded by `src/theme.rs`. Its contents are unchanged. RustRover can leave this extension as plain text (no web-CSS inspection) or use a compatible GTK syntax plugin. This trades browser-CSS completion for correct validation; it does not disable Rust or other file inspections. Previously saved inspection results may need to be rerun after project synchronization.

GTK itself validates the complete generated theme, including named colours and generated rail/accent rules. Run `./scripts/verify.sh` from Fedora. It runs formatting, build, strict Clippy, all ordinary tests, dedicated GTK stylesheet and focused-widget lifecycle tests, diff whitespace and Git object integrity checks. For the additional media/reflow/navigation suite use:

```sh
LCARS_MEDIA_FIXTURE=/home/fedora/lcars-test/test-pattern.webm ./scripts/verify.sh
```

Use an actual seekable test video; the suite uses its own temporary settings/directory fixtures. A display is required; the script uses Xvfb if no graphical display is set. The suite reports omitted media validation explicitly. GTK parsing errors fail the check, independent of IDE inspections.

Historical local backup directories are ignored, not removed. Do not force-push over an unrelated remote history. The configured remote currently has a separate `main` history while this project is on `master`; publish the project on a new branch and review how it should be integrated.

References: https://docs.gtk.org/gtk4/css-overview.html and https://www.jetbrains.com/help/idea/creating-and-registering-file-types.html

## Runtime repair and verification (2026-10-24)

The exact gtk_widget_is_ancestor invalid-widget critical was reproduced by focusing a Files row and entering its directory. GTK 4.10.5 focus traversal reached a removed widget. clear_focus_within now detaches root focus before removing dynamic file rows, mounted-volume controls and diagnostic findings, preserving focus elsewhere.

The regression repeats focused navigation 12 times and exercises panel transitions and background diagnostics. It failed with SIGTRAP before the fix and passes afterward with G_DEBUG=fatal-criticals. Final verification passed: 40 ordinary tests, three explicit GTK suites (theme, focus lifecycle, media/navigation), 30 page/viewport combinations, formatting, locked build, strict all-target Clippy, diff check and Git fsck. Two dangling Git blobs are harmless unreachable objects, not corruption. Theme bytes are unchanged.

Release rebuilt and installed at /home/fedora/.local/bin/lcars-desktop. Restart older running instances to use the fix. Physical touch and UTM cursor/suspend acceptance remain separate tracker gates.

## External Calculator TLS limitation

GNOME Calculator 44.0 cannot validate the ECB currency-rate certificate using the old Fedora 38 trust bundle. The same request succeeds using a current Mozilla CA bundle. Configured package repositories offer no newer ca-certificates package. Automatic approval review required explicit permission for adding the standard Sectigo E46 root; the user chose to leave system trust unchanged. No root was installed, and TLS verification remains enabled. Currency-rate download remains unresolved; this is not a Rust build error or Git push blocker.

## Publication handoff

Use the separate lcars-desktop branch. Local history and origin/main have no common ancestor; never force-push over main. A terminal push dry run failed because HTTPS credentials were unavailable. Sign in through RustRover GitHub integration or an approved Git credential flow, then push lcars-desktop. No remote publication has been performed. Authentication and repository write permission must be verified before claiming successful publication.
