# Live-delta inventory — 2026-09-06

Internal maintainer note. This is not a public knowledge-base entry.

## Scope and method

The starting catalog contained 253 TOML entries. This pass reviewed the newest
reusable material in LOG634–638, their project notes, the matching ledger
entries, and the older method sources they explicitly depend on. The comparison
was against entry IDs and full page content, not titles alone.

The review excluded puzzle narrative, ciphertext or plaintext material, live
target measurements, isolated implementation details with no reusable lesson,
and claims supported only by a currently running process. Four distinct
abstractions were accepted, taking the catalog to 257 entries.

## Accepted additions

| New entry | Why it is distinct | Evidence and status |
|---|---|---|
| `commit-before-reveal-evaluation` | `blind-controls` commits a hidden seed or performs an oracle-free search; `isolated-acquisition-solver-handoff` commits public input before a solver starts. The new page covers the opposite boundary: commit and independently verify *all solver outputs* before opening one hidden answer, with no answer registry and no trusted worker-verification flag. | LOG634's completed 12-case software boundary, 24 classifier paths, 13 refusal scenarios, strict event order, zero-read pre-authorisation failures, and consumed post-open failures. `VERIFIED` for the boundary only; no search or language-power claim. |
| `degenerate-input-exact-frontier-kat` | `kat-harness` lists KAT layers, while this page gives a construction for making the complete observable output set independently enumerable without bypassing the real model-bearing process. The constant-symbol reciprocal-map algebra and its power limitation were not documented elsewhere. | LOG637's one actual isolated process, two complete 26-map sets, 52 scalar inverse/forwards, actual model/runtime pins, and eight interface refusals. `VERIFIED` as an interface KAT, not full-frontier power. |
| `zero-aware-cartesian-product-cap` | Existing cap pages explain lineage, quotient-before-cap, and incomplete sentinels. None states the order-independent zero-factor rule or the overflow-safe product algorithm that prevents a large prefix product followed by zero from causing a false refusal. | Cited LOG609 exact enumerator and its preserved zero-factor regression; LOG636 independently replayed the over-cap whole-union withholding case. `VERIFIED` for cardinality and cap semantics. |
| `transitive-process-census` | Receipts bind files and the isolated-handoff page binds one parent/worker exchange. The new page treats a conditional multi-process pipeline as an independently reconstructed execution DAG and requires exact vertex/edge reconciliation, including no unsolicited child and no child after withholding. | LOG631 and LOG637 verify smaller actual process paths; LOG638's source-bound conditional-DAG verifier passed static refusal tests. The complete multi-branch preflight was still running at this cutoff, so the page is honestly `UNTESTED`. |

## Rejected as already covered

| Candidate lesson from LOG634–638 | Existing coverage | Decision |
|---|---|---|
| Symmetric second-order plaintext feedback and its 1,300 raw cells | `symmetric-second-order-plaintext-feedback`, `feedback-state-gauges-and-cells` | Reject duplicate. LOG635 adds release engineering, not new cipher algebra. |
| Identity plus every single coordinate-pair swap | `identity-plus-one-swap-neighborhood` | Reject duplicate. The 326-map application is another use of the documented neighbourhood. |
| Structural selection versus exact-text recovery | `parameter-versus-plaintext-recovery`, `endpoint-specific-control-contracts` | Reject duplicate. LOG635 applies the existing endpoint split. |
| Two matched maxima and plus-one permutation tail accounting | `permutation-test`, `family-wise-null`, `valid-model-nulls` | Reject duplicate. The concrete statistics are a pipeline configuration, not a new estimator. |
| Lexical winner, out-of-vocabulary bias, and pinned word model | `lexical-reranking-and-oov-bias`, `combined-character-word-models`, `n-gram-language-scoring` | Reject duplicate. |
| Physical aliases retained behind semantic rows | `candidate-bank-lineage-and-cap-accounting`, `conjugacy-and-gauge` | Reject duplicate. LOG634/636 provide more replay evidence for the same invariant. |
| Variable unique voters and exact denominators | `unique-proposal-voting` | Reject duplicate. |
| Partial involution distances, reflection phases, cycle roots, and complete physical phase expansion | `partial-involution-edit-distance`, `partial-reflection-phase-equality`, `partial-involution-conjugacy-query`, `reflection-product-cycle-lift`, `two-involution-component-templates` | Reject duplicate. |
| As-found file census is not historical provenance | `reproducibility-receipts`, `correction-and-retraction`, `ledger-reservations` | Reject near-duplicate. Those pages already distinguish current bytes from executed bytes and require contemporaneous receipts. |
| Durable `O_EXCL` attempt marker and no retry after a read | `blind-controls`, `seal-and-audit-protocol`, `kat-harness` | Reject duplicate. |
| Source, runtime, checkpoint, and transitive import pinning | `reproducibility-receipts`, `isolated-acquisition-solver-handoff`, `target-absent-controls` | Reject duplicate. The new process-census page addresses graph completeness rather than restating file pinning. |
| Fresh interval registry, no-redraw rule, and separate view evaluation | `held-out-gates`, `fresh-holdout-after-selector-change`, `blind-controls` | Reject duplicate. LOG638 records future-controller obligations but no new completed power result. |
| A complete top certificate cannot be replaced by one completed row | `bound-benchmarks-and-certificates`, `candidate-bank-lineage-and-cap-accounting` | Reject duplicate. |
| Wiring KAT versus recovery power | `known-answer-tests`, `kat-harness`, `planted-controls`, `power-measurement` | Reject duplicate. The degenerate-frontier page is accepted only for its exact fixture construction. |

## Evidence reviewed

Primary new notes and receipts:

- `withmath/docs/project-notes/PK89_DUAL634_GATE.md`
- `withmath/docs/project-notes/PK10_PLAINFEEDBACK635_SECONDORDER_LIVE_DESIGN.md`
- `withmath/docs/project-notes/PK10_PLAINFEEDBACK635_SECONDORDER_RELEASE_AUDIT.md`
- `withmath/docs/project-notes/PK89_DUAL636_REPLAY.md`
- `withmath/docs/project-notes/PK89_PREFIX637_PROCESS.md`
- `withmath/docs/project-notes/PK89_DUAL638_GATE.md`
- LOG634, LOG636, LOG637, and LOG638 entries in `withmath/COORDINATION.md`

Method sources followed because the new notes cite them:

- `withmath/docs/project-notes/PK9_VARVOTE607_KAT.md`
- `withmath/docs/project-notes/PK9_GROUP609_EXPAND.md`
- `withmath/docs/project-notes/PK9_EQUAL612_JOIN.md`
- `withmath/docs/project-notes/PK89_SELECTOR615_AUDIT.md`
- `withmath/docs/project-notes/PK89_DUAL622_GATE.md`
- `withmath/docs/project-notes/PK89_PIPELINE631_LIVE_RELEASE.md`
- `withmath/docs/project-notes/PK10_PLAINFEEDBACK630_SECONDORDER_THEORY.md`
- `withmath/docs/project-notes/PK10_PLAINFEEDBACK633_SECONDORDER_CONTROLS_V2.md`

## Cutoff and validation rule

The full LOG638 model-bearing multi-branch preflight was active when this audit
was taken. No intermediate census was promoted to a pass, cryptanalytic result,
or power claim. If it later terminates with a complete independently replayed
receipt, `transitive-process-census` may be changed from `UNTESTED` only after
reviewing that terminal record and its exact source tuple.

Validation for the four accepted pages requires strict catalog parsing, math
span checking, list-marker checking, and whitespace checking. The command
results are recorded in the completing agent's handoff rather than invented in
this file before they run.
