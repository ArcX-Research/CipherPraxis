# Cipher-method and cross-cipher inventory — 2026-09-05

Internal maintainer note. This file is not a public knowledge-base entry.

## Scope and method

This bounded pass started from the 234-entry catalog. It read
`AUTHORING.md`, `CONTENT_GAPS.md`, and `DEEP_INVENTORY_2026-09-05.md` in full,
then compared the current entry text with all 28 notes in
`withmath/docs/cipher-methods/` and all 97 notes in
`withmath/docs/cross-cipher/`. The source sweep checked definitions,
equations, implementation claims, controls, limitations, and later audits of
the same lane. File families below are grouped only where the notes are
successive runs of one method; the counts account for all 125 source notes.

The pass was capped at six entries. A candidate was accepted only when it was
a reusable abstraction missing from the catalog and had a stable derivation,
software, and held-out or exhaustive check. A family-specific application,
later compute receipt, negative result, or alternate name for an existing page
was recorded here instead of receiving another public entry.

## Added entries

| Entry | Why it is distinct | Evidence and status |
| --- | --- | --- |
| `fractionated-morse` | The transform appeared only as a short neighboring-family note under `compressocrat`. The catalog did not explain ternary Morse grouping, invalid-token handling, or the observable-table equivalence caused by unused ranks. | `COLOSSUS_DYNAMIC_AUDIT.md`, `wm_colossus_dynamic.py`, `wm_colossus_fracmorse_equiv.py`, and log 186b. `VERIFIED` for the transform and observable-class control. |
| `interrupted-key` | `variant-beaufort` mentioned the reset variant but did not define the state update, plaintext/ciphertext reset distinction, attack, or timing failure modes. | `COLOSSUS_DYNAMIC_AUDIT.md`, `wm_colossus_dynamic.py`, and the interrupted-key log. `VERIFIED` on Vigenere, Variant, and Beaufort controls across the declared selector. |
| `nicodemus` | No existing entry covered a periodic substitution followed by a column readout that restarts at every small block, including its ragged final-block geometry. | `COLOSSUS_DYNAMIC_AUDIT.md`, `wm_colossus_dynamic.py`, and the Nicodemus log. `VERIFIED` on all three additive sign conventions. |
| `mutual-information-column-order` | `order-free-width-statistic` chooses a width from class patterns. This method instead ranks the order of already formed chunks by a contingency statistic unchanged by separate unknown symbol relabelings. | `SHARED9_FACTOR_DEAL_AUDIT.md`, `wm_shared9_factor_deal.py`, and logs 202a–d. `VERIFIED` as a top-256 order screen: the hidden order was retained on four of four controls. |
| `relax-then-project-decoding` | Existing assignment and exact-completion pages explain individual tools, but not the complete decomposition: fit independent literal maps for language, convert reliable edges to one shared cyclic/dihedral system, restore gauges, and replay the original model. | `SHARED9_FACTOR_AUDIT.md`, `SHARED9_FACTOR_DEAL_AUDIT.md`, their solvers, and logs 198/202. `POWER-LIMITED`: exact on dense controls, unreliable for some short or mixed groups. |
| `cascade-cancellation` | Pairwise difference pages cancel one shared stream. This derives the signed three-stream combination that eliminates two intermediate running texts in a two-hop chain and leaves a periodic endpoint. | `CROSSCASCADE_AUDIT.md`, `wm_crosscascade.py`, its frozen runner, and log 214. `VERIFIED` on two full-selector controls with exact endpoint, key, and all forward checks. |

This lane added six entries. Immediately after the additions, the catalog had
240 TOML files and 240 entries.

## Cipher-method notes assessed

| Source notes | Count | Apparent reusable gap | Decision |
| --- | ---: | --- | --- |
| `AFFINE_ROUTE_AUDIT.md` | 1 | Affine position routes and their exact parameter gauge | Already covered by `route-transposition`, `affine-permutation-maps`, and `conjugacy-and-gauge`. |
| `AUTOKEY_AUDIT.md` | 1 | Plaintext autokey mechanics and controls | Already covered by `autokey`; the audit adds a scoped run, not a second method. |
| `COLOSSUS_AUDIT.md` | 1 | Direct 5-by-5 fractionation and digraphic families | Bifid, Trifid, Playfair, Two-square, and Four-square already have pages. The external-family wrapper is implementation provenance, not a general primitive. |
| `COLOSSUS_DYNAMIC_AUDIT.md` | 1 | Gromark, Condi, Fractionated Morse, Interrupted Key, and Nicodemus | Gromark and Condi already have full pages. Fractionated Morse, Interrupted Key, and Nicodemus were genuinely missing and were added. |
| `COMPRESSOCRAT_AUDIT.md` | 1 | Compressocrat state and search | Already covered by `compressocrat`; its Fractionated-Morse aside was not enough to replace the new dedicated page. |
| `CONDI_EXACT_AUDIT.md`, `CONDI_GLOBAL_AUDIT.md`, `CONDI_PARITY_AUDIT.md` | 3 | Exact inference, global-basin repair, parity-first CRT lift | The cipher is covered by `condi`; CRT, permutation search, power gates, and observable parameters are covered separately. These are successive solver-power studies. |
| `CYCLEBEAM_EXACTPREFIX_AUDIT.md` | 1 | Exact prefix feasibility inside a cycle beam | Already represented by `shared-cycle-attack`, `exact-chain-oracle`, `word-prefix-csp`, and `all-different-enumeration`. |
| `CYCLEWORD_AUDIT.md`, `CYCLEWORD_SEMIMARKOV_AUDIT.md` | 2 | Whole-word and exact-length semi-Markov search | Already covered by `word-lattice-segmentation`, `combined-character-word-models`, and `shared-cycle-attack`. The notes measure decoders rather than introduce a new language model. |
| `DIGRAPHIC_SLIDES_AUDIT.md` | 1 | Portax and Slidefair | Already covered by `digraphic-slide` and its related cipher pages. |
| `GROMARK5_AUDIT.md`, `GROMARK67_RECURRENCE_AUDIT.md` | 2 | Longer primers, recurrence splitting, and CRT meet-in-the-middle | Already covered by `gromark`, `linear-recurrences-lcg`, `modular-inverse-and-crt`, and `meet-in-the-middle`. |
| `HILL2_PERIODIC_AUDIT.md` | 1 | Hill-2 under a periodic additive layer | The components and their composition are covered by `hill-cipher`, `periodic-substitution-dp`, and `multiple-rounds-and-composition`. This is one family placement. |
| `PERIODIC_AFFINE_DP_AUDIT.md`, `PERIODIC_PORTA_DP_AUDIT.md` | 2 | Exact cyclic decoding of periodic affine and Porta maps | Cipher mechanics and the reusable decoder are already covered by `affine-substitution`, `porta`, and `periodic-substitution-dp`. |
| `QIV_GAP_AUDIT.md` | 1 | Quagmire IV crib constraint search | Already covered by `quagmire-iv`, `crib-csp`, `constraint-satisfaction-forward-checking`, and `all-different-enumeration`. The note closes an execution-history question. |
| `QTT_RANGE_SHARD_ENGINEERING_AUDIT.md` | 1 | Exact rank intervals, local pruning, and deterministic reduction | Already covered by `rank-sharding`, `bound-benchmarks-and-certificates`, `reproducibility-receipts`, and deterministic tie handling in the engineering pages. |
| `SHARED9_CYCLE_AUDIT.md` | 1 | Direct shared-cycle, shared-period model | The reusable family and its identifiability limit are covered by `shared-cycle-attack`, `conjugacy-and-gauge`, and `parameter-versus-plaintext-recovery`. |
| `SHARED9_FACTOR_AUDIT.md` | 1 | Independent literal maps followed by exact common-action projection | Accepted in general form as `relax-then-project-decoding`; no width-specific duplicate was added. |
| `SHARED9_FACTOR_DEAL_AUDIT.md` | 1 | Relabeling-invariant order score and ensemble projection under a shared deal | The mutual-information screen was accepted. The projection stage was folded into `relax-then-project-decoding`; the shared width is an application. |
| `SHARED9_GAP_REAUDIT.md` | 1 | Coverage certificate for prior shared-width runs | Useful internal coverage record, but not a cryptographic method. |
| `SHARED9_GLOBAL_CONCAT_CONTROL_AUDIT.md` | 1 | Global concatenation adapter and duplicate collapse | The adapter applies `shared-cycle-attack`; duplicate handling and controls are covered by `candidate-bank-lineage-and-cap-accounting` and `endpoint-specific-control-contracts`. |
| `SHARED9_OUTER_AUDIT.md` | 1 | Shared outer column order and two deal geometries | Already represented by `columnar-transposition`, `interleaved-transposition`, `order-free-width-statistic`, and `sandwich-construction`. |
| `SHARED_CYCLE_CHAIN_EQUATION_AUDIT_CLAUDE.md` | 1 | Partial chain equations, CRT rank, all-different feasibility, and gauge rules | Already covered by `exact-chain-oracle`, `all-different-enumeration`, `modular-inverse-and-crt`, and `conjugacy-and-gauge`. It is an independent audit, not another technique. |
| `TRIFID_AUDIT.md`, `TRIFID_Q_AUDIT.md` | 2 | Trifid and Trifid plus an outer periodic layer | Already covered by `trifid`, `periodic-substitution-dp`, and `multiple-rounds-and-composition`. |

## Cross-cipher notes assessed

The grouped counts in this table sum to all 97 notes in
`withmath/docs/cross-cipher/`.

| Source notes | Count | Apparent reusable gap | Decision |
| --- | ---: | --- | --- |
| `CROSSCASCADE_AUDIT.md` | 1 | Cancel two hidden intermediate streams in a three-stream linear cascade | Accepted as `cascade-cancellation`. |
| `CROSSCHAIN*.md` | 14 | Exact chain equations; modulus-13 validation and rank-sharded production receipts | Core algebra already appears in `exact-chain-oracle`, `all-different-enumeration`, and `residue-class-graphs`. Rank and run engineering are covered by `rank-sharding`, `reproducibility-receipts`, and `candidate-bank-lineage-and-cap-accounting`. |
| `CROSSCOLUMN*.md` | 5 | Column, row, vertical, and measured cross-source placements | Applications of `columnar-transposition`, `sandwich-construction`, `transposition-width-recovery`, and existing order-free statistics. No new invariant survived the later measured audits. |
| `CROSSDERIVE_ZONE_EXACT_CONTROL_AUDIT.md`, `CROSS_DERIVED_STRUCTURAL_COVERAGE_AUDIT.md` | 2 | All-pair derived streams with exact periodic endpoints | Already covered by `difference-operators`, `key-cancellation`, `running-key-relation-test`, `exact-period-alignment`, and `periodic-substitution-dp`. |
| `CROSSELIMINATE_AUDIT.md`, `CROSSELIM_*.md` | 5 | Elimination, global search, parity/CRT decomposition, and word scoring | One-hop elimination is already `difference-operators` plus `key-cancellation`; parity lifting and language search are covered elsewhere. The two-hop extension is the distinct `cascade-cancellation` page. |
| `CROSSPAIR_INVARIANT265_AUDIT.md` | 1 | Same-plaintext pair invariants under a common action | Already covered by `difference-operators`, `residue-class-graphs`, and `shared-cycle-attack`. |
| `CROSS_DOUBLE9_PROFILE_AUDIT.md`, `CROSS_DUAL_PRETRANSPOSE_AUDIT.md` | 2 | Two width-9 layers and dual pre-transpositions | Family placements composed from existing transposition, sandwich, and periodic-layer pages. The profile audits are power measurements. |
| `CROSS_PRESOURCE_QT*.md` | 12 | Pre-source substitution/transposition order, balanced search, certificates, resumed ranges, and production receipts | The reusable pieces are already `sandwich-construction`, `maximum-weight-bipartite-assignment`, `partial-permutation-completion-bounds`, `bound-benchmarks-and-certificates`, `cp-sat-and-smt-solvers`, and `rank-sharding`. |
| `CROSS_PRESOURCE_TQ*.md` | 8 | Reverse layer order, assignment bounds, CP-SAT, and refinement | Same existing abstractions as the QT lane. The successive bound benchmarks include useful evidence already carried by the bound pages, not a new cipher behavior. |
| `CROSS_PRETRANSPOSE*.md` | 6 | One pre-transposition, global search, controls, and measured production | An application of `columnar-transposition`, `sandwich-construction`, `shared-cycle-attack`, and controls-first validation. |
| `CROSS_Q7_T9_PLACEMENT_MATRIX.md` | 1 | A matrix of layer orders and shared versus independent placements | Kept as an internal coverage taxonomy. It has no independent transform or validated inference rule, so a public page would be family-specific bookkeeping. |
| `CROSS_QTT*.md` | 5 | A substitution between two transpositions, affine restrictions, and hierarchical exact search | Already composed from `sandwich-construction`, `multiple-rounds-and-composition`, assignment bounds, partial completion, and rank-sharding. The hierarchy is a search plan for one family. |
| `CROSS_SANDWICH9*.md` | 2 | Width-9 sandwich and k-best handoff | Already covered by `sandwich-construction`, `k-best-lists`, and the transposition/order pages. |
| `CROSS_SERIAL_PAIRWISE_SUCCESSOR_STATIC_AUDIT.md` | 1 | A serial pairwise successor construction | Static design and power analysis did not provide a qualified reusable decoder. The construction uses existing cross-chain and periodic tools. Deferred rather than promoted. |
| `CROSS_SERIAL_PRETRANSPOSE267_AUDIT.md` | 1 | Serial pre-transposition combined with chain constraints | A family-specific composition of the pre-transposition and exact-chain machinery already represented. |
| `CROSS_T9Q7*.md` | 5 | Transposition-before-periodic layer, profiles, measurements, and k-best search | Covered by `sandwich-construction`, `order-free-width-statistic`, `periodic-substitution-dp`, and `k-best-lists`. |
| `T9Q7_AUDIT.md` | 1 | Earlier form of the same transposition/periodic family | Superseded by the measured family above and represented by the same existing pages. |
| `CROSS_ZONE_CYCLE_CORPUS_MEET*.md` | 5 | Corpus-token anchors joined against a shared cycle, including independent audits | The general method is already in `shared-cycle-attack`, `corpus-bank-screen`, `meet-in-the-middle`, and `exact-chain-oracle`. Later notes are controls and production receipts. |
| `CYCLEBIMEET_AUDIT.md` | 1 | Bidirectional meet for a common cycle | Already covered by `meet-in-the-middle` and `shared-cycle-attack`; the note is a family-specific decoder. |
| `DOUBLE9_Q7_AUDIT.md` | 1 | Double width-9 transposition around a periodic layer | A specific multi-round composition already described by transposition, sandwich, and periodic pages. |
| `ELEMENT_CROSSCHAIN*.md` | 10 | Finite vocabulary automata, compact exact models, CP-SAT/Z3, incremental search, and pattern bounds | Already covered by `word-prefix-csp`, `cp-sat-and-smt-solvers`, `constraint-satisfaction-forward-checking`, `exact-chain-oracle`, and `partial-permutation-completion-bounds`. Vocabulary choice is application data, not a new primitive. |
| `MIXEDPERIOD_GRAPH*.md` | 4 | Join equations with mixed periods as a residue graph | Already covered by `residue-class-graphs`, `exact-chain-oracle`, and `compound-periodic-key`. Later notes are production ranges. |
| `OBJECT_VOCAB_EXACT_AUDIT.md` | 1 | Exact enumeration over a small object vocabulary | Already covered by `corpus-bank-screen`, `word-prefix-csp`, and exact enumeration. A particular word bank is not a general method. |
| `QTT_RANK_CONSTRUCTOR_FEASIBILITY_AUDIT.md` | 1 | Construct instances at chosen exact rank for power tests | The general lesson is already in `power-analysis`, `rank-sharding`, and `endpoint-specific-control-contracts`. The constructor is tied to one family. |
| `TWIN_BIFID_Q7_AUDIT.md`, `TWIN_TRIFID_AUDIT.md` | 2 | Shared fractionation structures across two sources | Cipher mechanics are already in `bifid` and `trifid`; shared-parameter testing is covered by `shared-cycle-attack` and multi-source validation pages. |

## Important non-additions

### Operator-placement matrix

`CROSS_Q7_T9_PLACEMENT_MATRIX.md` is valuable as an internal checklist: it
prevents a team from assuming that testing `T(Q(P))` also tested `Q(T(P))`, or
that a shared layer also tested independent layers. It was not made a public
entry because the current evidence is a catalog of one construction family,
not a general reduction, statistic, or validated algorithm. A future page on
operator-order coverage would need examples from several unrelated families
and a clear control protocol.

### Exact endpoint after a bounded selector

Several cross-chain and pre-source notes finish a shortlisted cell exactly.
That does not make the complete family search exact. The distinction is
already explicit in `parameter-versus-plaintext-recovery`,
`partial-permutation-completion-bounds`, `rank-sharding`, and
`endpoint-specific-control-contracts`. No new “exact cross search” page was
created.

### More pages for negative runs

Repeated larger budgets, resumed rank intervals, or independent audits are
important evidence for existing pages. They are not new cryptographic
knowledge by themselves. This inventory records them through the grouped
source families without turning each run into a method.

## Verification

Verification results from this lane:

- Every provenance path in the six new TOML files exists under
  `../Cryptanalysis`.
- Strict content check: 240 files, 240 entries, zero errors, zero warnings.
- Math scan: zero unhandled dollar signs.
- List scan: zero broken list markers.
- `git diff --check` passed for tracked work; an equivalent no-index check
  produced no whitespace diagnostics for each new file.
- No existing TOML, existing inventory, UI/Rust file, generated asset, or user
  file was edited, and no commit was made.
