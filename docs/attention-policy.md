# Automatic task emphasis — Step 05 in progress

Task demand is emitted explicitly by media selection and the start/end of transfer and diagnostic work. It does not parse status strings or react to CPU/network telemetry. Demand is transient and not serialized. The most recently started eligible task wins; repeated starts are idempotent, ending it exposes the previous active task, and hidden systems are never reopened by demand.

The GTK adapter consumes this demand only on the automatic home layout. It does not change the focused application. Solver emphasis gives the task a larger target area while retaining readable minima for the other systems and the emblem. The un-emphasized composition is kept in memory and restored when no tasks need space; persistent layout writes use the baseline rather than saving temporary geometry. Entering Arrange restores baseline geometry and protects manual layout.

Normal click holds and active text focus defer demand updates. This is an initial guard, not proof of multi-contact touchscreen handling. Manual arrangements currently do not receive automatic task emphasis. The remainder of Step 05 must validate full input sequencing, keyboard occlusion, alerts and restoration with concurrent tasks/viewport changes. Reduced-motion wiring remains explicitly tracked under Step 06.

Tests cover demand precedence, idempotence, completion, hidden systems and live GTK temporary enlargement/restoration without focus stealing. Existing continuity checks cover real Files state and a diagnostic worker completing while hidden. Transfer lifecycle is wired but no live network speed test is automatically launched by these checks.

The existing workspace reducer remains the shared event specification; persistent GTK widgets still own their content. Demand owns only ephemeral task priority and does not copy paths, selections, jobs or widget state into a competing model.
