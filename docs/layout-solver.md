# Content-aware layout solver — Step 03

src/layout.rs is a pure logical-pixel solver consuming the six workspace content contracts. It considers row partitions and emblem insertion positions (448 candidates for six panels plus emblem), weighted area balance, preferred dimensions/aspect, central emblem exposure and overflow. Panel order is deterministic; the same inputs produce the same geometry. Attention gives one panel additional weight without hiding supporting panels. Focus produces one full-workspace panel.

Hard minimum content sizes include a 48px title-control allowance. If all systems cannot fit, canvas height grows rather than shrinking text; horizontal overflow is allowed only when even one panel's minimum width cannot fit. Renderer integration must expose that virtual canvas through a scroller instead of scaling it back down. An empty desktop gives the emblem the viewport. Invalid focus outside the visible set is ignored; duplicate panel IDs are deduplicated.

Small resizes retain the previous partition when its score is within the documented tolerance and canvas size matches the best candidate. This prevents switching between nearly identical compositions. Manual geometry remains separately owned and must not be overwritten by temporary attention layouts.

The invariant test matrix covers all 64 system subsets at eight viewport sizes/aspect ratios (including zero, portrait, narrow and ultrawide), checks deterministic results, readable minima, bounds and non-overlap. Additional tests cover focused layout, dense/equal idle composition, attention growth, topology stability and overflow fallback. A narrow-case test caught 8px of avoidable horizontal scrolling and led to the explicit width constraint.

This is solver completion, not final responsive UI completion. Step 04 must adapt the GTK surface, viewport scrolling, persistent page state and content rendering; Step 05 connects real task activity; Step 06 refines coordinated animation/hit regions. Initial minima/preferred sizes remain tunable after real content checks.
