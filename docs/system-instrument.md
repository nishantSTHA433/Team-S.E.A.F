# Dedicated Console system instrument

Console overview now renders its System region directly at the available size, rather than shrinking the Desktop dashboard. It contains a local clock/date, up to 60 live CPU samples, vertical CPU and RAM utilization meters, home-filesystem usage, uptime and receive/transmit rates. It shares the existing telemetry feed; no second monitoring worker or network probe was added.

HOME refers to the filesystem containing /home, matching the existing df query; it is not assumed to be the C: root mount. Unknown memory/storage values show --. Data older than 20 seconds is labeled stale. These are Fedora guest readings.

Selecting System still opens the detailed view. All Systems returns to the dedicated instrument. Desktop mode retains its dashboard. The overview itself is read-only, with the existing large selection target covering it.

Validation: build, strict Clippy and ten existing tests passed; visually checked the overview clock and readings updating, expanded System, and returned to overview.

Next: dedicated network/diagnostic instruments with real traffic histories; a touch calculator keypad; compact native-app launcher. Preserve BR-01–BR-14, media inspection, saved arrangements, and the Desktop/Console preference. Physical touch validation and smooth zoom animation remain outstanding.
