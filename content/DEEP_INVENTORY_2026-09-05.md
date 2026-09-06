# Deep content inventory — 2026-09-05

Internal maintainer note. This file is not a public knowledge-base entry.

## Scope and method

This pass started from the 228-entry catalog and read `content/AUTHORING.md`,
`content/CONTENT_GAPS.md`, the relevant architecture sections, the Vigenère
exemplar, and the complete 517-line project-notes index. It then followed the
index into the late exact-algebra notes for logs 550–633 and compared each
candidate abstraction with the current entry IDs and text, not just their
titles. Search terms included schedules, reflections, semiregularity,
conjugacy, component topology, edit distance, aliases, restricted-growth
strings, caps, nulls, transfer, and exact completion.

The pass was deliberately capped at six new entries. A page was added only
when the corpus supplied a stable derivation plus exact software checks or a
matched control receipt. General group-theory or graph-theory facts are marked
as literature-known inside the page; the specific reduction, oracle, or
control contract is described as project-developed. No page claims that a
validated algebraic primitive alone has plaintext-recovery power.

## Added entries

| Entry | Why it is distinct | Evidence and status |
| --- | --- | --- |
| `alternating-velocity-disk-cipher` | A disk schedule with two alternating velocities is neither constant advance nor constant acceleration outside the exact differences 0 and 13. It was absent from `alberti-progressive`. | `PK10_PERIOD2_616_CONTROLS.md`, LOG616, independent LOG618 reconstruction, and LOG621 result audit. `VERIFIED` for the declared selector/control family; the production measurement was ambiguous. |
| `reflection-product-cycle-lift` | Existing cycle and dihedral pages explain that two reflections multiply to a rotation, but not the exact inverse problem: recover every compatible regular cycle from one product, including the explicit prime two-orbit root list and degenerate deferrals. | `PK8910_THEORIST550_GAP.md`, `wm_dihedral552_exact.py`, LOG552. `VERIFIED` as a complete-map algebra oracle. |
| `semiregular-common-cycle-criterion` | Pairwise legal cycle types and commutation are insufficient for three or more reflections. Semiregularity supplies the missing mixed-word condition and is an exact complete-map criterion in the stated degree. | `wm_dihedral556_joint.py`, LOG556. `VERIFIED` on exhaustive small degree, planted degree 26, and adversarial cases. |
| `partial-involution-conjugacy-query` | Existing conjugacy and all-different pages do not give the exact open-endpoint and unused-orbit capacity test for a partial relabelling, nor its compressed OR over original partial maps. | `PK9_CONJUGACY570_EXACT.md`, LOG570. `VERIFIED` against full-completion enumeration. |
| `partial-involution-edit-distance` | Bounded permutation shells define a search region but do not compute the exact minimum number or cost of observed reciprocal components that must be removed to enter a conjugate-completion family. | `PK9_EDIT573_EXACT.md`, LOG573. `VERIFIED` against exact extension enumeration, including weighted cases. |
| `two-involution-component-templates` | The general permutation-cycle page treats one partial directed cycle. This theorem treats two coloured involutions jointly and reduces their completion to one alternating path, one alternating cycle, or two prime-sized paths. | `PK9_COMPONENT562_EXACT.md`, LOG562/563. `VERIFIED` for the component algebra and implementation parity; the downstream language decoder remains power-limited. |

The catalog now contains 234 entries.

## Apparent gaps rejected as duplicates

### `additive-stream-wheels` (original decision, superseded 2026-09-06)

The canonical ID is absent, but the abstraction is already present under the
more precise pages `compound-periodic-key` and `lcm-of-periods`. The evidence
in `PK3_INSTRUMENT_METHOD.md` is exactly their stated rule: two additive
periodic streams in one alphabet combine into one effective stream whose
period divides the least common multiple, with separate gauge constants when
the component-period graph is disconnected. Adding `additive-stream-wheels`
would create a second name for the same model, not preserve new knowledge. A later pass narrowed
the missing abstraction to **structured source decomposition** rather than layer composition: it
now records signed and phased source wheels, exact-zero-residual verification, decomposition
gauges, and family-wide source-bank nulls. That narrower page was added without duplicating the
two-alphabet layer solver.

### Restricted-growth-string schedule classes

LOG619 gives an especially useful exact class census for a constant input,
but the reusable abstraction—canonical equality patterns as observable
function fingerprints, with raw aliases kept behind each class—is already
covered by `observable-function-fingerprinting`,
`latent-label-identifiability`, and `feedback-state-gauges-and-cells`. The
period-two cipher page cites the census where it belongs; no fourth generic RGS
page was created.

### Exact retained-group expansion and alias caps

LOG609's checked Cartesian-product count, canonical cyclic-group aliases,
deterministic top-K, and cap refusal are already covered by
`exhaustive-enumeration`, `candidate-bank-lineage-and-cap-accounting`,
`conjugacy-and-gauge`, and `bound-benchmarks-and-certificates`. Its reflection
formula is an application of the new lift and component pages, not a separate
general method.

### Projection-table pruning

LOG563's table of supported partial images is a useful implementation of a
standard extensional table constraint. The reusable rule—reject a partial
assignment when no bank row supports it—is already in
`constraint-satisfaction-forward-checking` and `cp-sat-and-smt-solvers`.
Creating a page for one four-image table would over-specialise the catalog.

### Target-conditioned and full-selector nulls

The late release notes repeatedly require one fixed selector across the
observation and every permutation, with the family maximum recomputed in each
replicate. This is already stated in `valid-model-nulls`, `family-wise-null`,
`permutation-test`, and `selection-overfit-diagnostic`.

### Stage failure labels

The useful distinction among missing proposal support, budget allocation,
candidate retention, final selection, exact text, and replay is already split
across `endpoint-specific-control-contracts`,
`candidate-bank-lineage-and-cap-accounting`, `power-analysis`, and
`parameter-versus-plaintext-recovery`. A new umbrella page would repeat those
contracts.

### Complete-token anchor search

The exact anchor-and-join construction and its Cartesian-product obstruction
are already recorded in `shared-cycle-attack`, including the scratch power
failure. It remains a power gap, not a missing method page.

### Range and fixture audits

Half-open corpus reservoirs, exclusion distances, fresh seeds, and semantic
replay are already covered by `held-out-gates`, `seed-determinism`, and
`reproducibility-receipts`.

## Canonical ID later filled from literature

### `nihilist`

The ID appears in the authoring guide's canonical list, but a project-wide
non-log search found no Nihilist-cipher derivation, implementation, planted
control, exact test, or scoped result. Polybius and additive ciphers exist
separately in the catalog, but combining their descriptions without an
evidence record would have invented project coverage in this control-focused
pass. A later literature lane added the standard construction from cited
classical sources, explicitly without claiming a project solver or control
result. That resolves the missing knowledge page while preserving this
original evidence finding.

## Deferred application, not a seventh page

The observed-component list decoder in LOG555 is a real cryptanalytic
application: delete a bounded number of reciprocal components, complete the
two involutions, lift their common cycle, use an admissible disagreement
bound, and refit the remaining phases. Its reusable machinery is now captured
by `partial-involution-edit-distance`, `two-involution-component-templates`,
`reflection-product-cycle-lift`, and the existing `shared-cycle-attack` and
`k-best-lists` pages. It recovered two consumed development controls, but no
fresh integrated gate exists. A separate application page should wait for
that gate rather than restating the same primitives as a verified attack.

## Verification

- Every new provenance path was checked to exist under `../Cryptanalysis`.
- Strict content check: 234 files, 234 entries, zero errors, zero warnings.
- Math scan: zero unhandled dollar signs.
- List scan: zero broken list markers.
- `git diff --check` passed for all six new TOML files.
- No existing TOML, `CONTENT_GAPS.md`, UI file, generated asset, or user file
  was edited, and no commit was made.
