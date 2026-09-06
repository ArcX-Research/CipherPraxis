# Mathematical and validation inventory — 2026-09-05

Internal maintainer note. This file is not a public knowledge-base entry.

## Scope

The pre-write comparison covered all 249 TOML articles then present in `content/`. After adding the four entries below, a final overlap review covered the current 253-entry catalog. That review included all five literature/reference pages and the new `permutation-keystream`, `homophonic-substitution`, and `coordinate-columnar-fractionation` articles added concurrently on 2026-09-05.

The catalog was compared with the reusable methods recorded in:

- `withmath/docs/pk8/` — 2 notes
- `withmath/docs/pk9/` — 41 notes
- `withmath/docs/pk10/` — 43 notes
- `withmath/docs/pk8-pk9/` — 21 notes
- `withmath/docs/pk9-pk8/` — 20 notes
- `withmath/docs/search-language/` — 8 notes
- `withmath/docs/project-notes/` — 97 notes
- durable method and result entries in `withmath/COORDINATION.md`

The source set contains 232 Markdown notes plus the coordination ledger. The review used article IDs, titles, summaries, equations, controls, and provenance—not filenames alone. Public articles avoid the source project's narrative and keep only general cryptographic ideas.

## Accepted additions

### `columnwise-hamming-radius-invariant`

Why it is separate: equality-pattern analysis tests whether a proposed plaintext preserves symbol-equality classes. This method instead optimises one global template under maximum Hamming distance and proves that the optimum is unchanged by a different bijection in every column. The observed-symbol restriction and matched-profile diagnostic are also specific to this objective.

Evidence boundary: the isometry and finite optimisation are exact. A radius above a fixed budget rejects that template model. A compatible radius is not positive evidence without a matched-profile null.

Primary receipts: `PK9_TEMPLATE_RADIUS428_NOTE.md`, `wm_pk9_templateradius428.py`, and the LOG428 transcript.

### `full-residue-anchor-enumeration`

Why it is separate: the dictionary and word-prefix articles describe finite banks and incremental crib constraints. This method states the stronger completeness theorem: one anchor visiting every periodic residue determines the whole key, so visiting every frozen anchor visits every key that can satisfy the declared finite language.

Evidence boundary: complete only for the frozen alphabet, relation, period, phase, anchor geometry, and bank. It does not show that the chosen bank describes an unknown message.

Primary receipts: the LOG379 design and independent audit, plus the LOG383 implementation and LOG391 audit.

### `translated-set-packing`

Why it is separate: the existing all-different pages solve letter-coordinate systems through CRT or generic constraint search. This method groups a full-alphabet window by periodic-key residue and solves it as pairwise-disjoint translations of whole sets. Its common-shift gauge and Beaufort/Vigenère feasibility equivalence are part of the reduction.

Evidence boundary: exact for the stated additive all-different window. It says nothing about language, symbol order, other periods, or another cipher layer.

Primary receipts: `PK9_RAWPERM_ZERO279_AUDIT.md`, the LOG279 solver, and its complete structural-zero transcript.

### `partial-reflection-phase-equality`

Why it is separate: partial-involution conjugacy and edit distance compare one partial map with a fixed template. This method compares two phases inside one regular cyclic action. One shared image forces equal phase; noisy partials then give an exact equal-phase deletion cost and a safe lower bound for distinct phases.

Evidence boundary: conditional on a common regular cycle, reciprocal partial maps, an immutable prefix, and component-count distance. The distinct-phase bound is necessary, not sufficient.

Primary receipts: `PK9_EQUAL610_COLLAPSE.md`, the LOG610 kernel and KAT, and the LOG610 receipt.

## Existing coverage retained

The following source ideas were already represented well enough and did not need another article:

| Source idea | Existing article or articles |
|---|---|
| Equality patterns under unknown symbol labels | `isomorph-analysis`, `observable-function-fingerprinting`, `latent-label-identifiability` |
| Repeating-key residue equations and collision graphs | `residue-class-graphs`, `all-different-constraints`, `exact-period-alignment` |
| CRT factorisation of permutation constraints | `modular-inverse-and-crt`, `all-different-enumeration` |
| First-return cycle masks and legal permutation orbits | `permutation-cycles`, `semiregular-common-cycle-criterion` |
| Two-reflection component geometry | `two-involution-component-templates`, `reflection-product-cycle-lift` |
| Partial involution completion and error radius | `partial-involution-conjugacy-query`, `partial-involution-edit-distance`, `partial-permutation-completion-bounds` |
| Cross-text cancellation and chained layers | `key-cancellation`, `segment-chain-analysis`, `cascade-cancellation`, `sandwich-construction` |
| Exact periodic decoding | `periodic-substitution-dp`, `viterbi-decoding`, `exact-dynamic-programming` |
| Word and object banks | `dictionary-attack`, `corpus-bank-screen`, `word-prefix-csp`, `word-lattice-segmentation` |
| Chunk ordering by boundary evidence | `mutual-information-column-order`, `transposition-width-recovery`, `route-transposition` |
| Local-search basin and shortlist loss | `polish-basin-measurement`, `k-best-lists`, `beam-search`, `relax-then-project-decoding` |
| Neural and lexical combination | `neural-language-model-decipherment`, `combined-character-word-models`, `lexical-reranking-and-oov-bias` |
| Model fit versus solver power | `family-identification`, `power-analysis`, `power-measurement`, `parameter-versus-plaintext-recovery` |
| Full-selector calibration | `family-wise-null`, `permutation-test`, `selection-overfit-check` |
| Within-residue and conditional nulls | `within-coset-null`, `valid-model-nulls`, `shuffled-null-z-score` |
| Control boundary variants | `planted-controls`, `endpoint-specific-control-contracts`, `fresh-holdout-after-selector-change` |
| Exact zero and bounded-search certificates | `exhaustive-enumeration`, `bound-benchmarks-and-certificates`, `tautological-gate-rule`, `throughput-fail` |
| Alias-safe grouping, caps, and lineage | `candidate-bank-lineage-and-cap-accounting`, `unique-proposal-voting`, `conjugacy-and-gauge` |
| Semantic replay versus byte reproducibility | `reproducibility-receipts`, `seal-and-audit-protocol` |

## Rejected duplicate candidates

- **Stratified full-selector nulls.** Already covered by `permutation-test`, `within-coset-null`, `family-wise-null`, `valid-model-nulls`, and the null section of `segment-chain-analysis`. The source's finest-intersection strata are a good worked design, not a separate general method.
- **Boundary-control matrices.** Already explicit in `planted-controls` and `endpoint-specific-control-contracts`.
- **Necessary-condition zero certificates.** The exact claim grammar, full census, and incomplete-versus-zero distinction already live in `exhaustive-enumeration`, `bound-benchmarks-and-certificates`, and `tautological-gate-rule`.
- **Unknown/time-limited states must survive.** Already covered in `beam-search`, `meet-in-the-middle`, `cp-sat-and-smt-solvers`, `throughput-fail`, and `candidate-bank-lineage-and-cap-accounting`.
- **Class-first multi-seed pooling.** Already covered by `candidate-bank-lineage-and-cap-accounting`, `unique-proposal-voting`, `latent-label-identifiability`, and `gauge-defect-invalidation`.
- **Semantic transcript comparison.** Already covered by `reproducibility-receipts`; fresh transcripts with timing fields need semantic comparison rather than whole-file hash equality.
- **Order-invariant chunk scoring.** Already covered by `mutual-information-column-order` and `transposition-width-recovery`; the source implementation is one specialised selector.
- **Finite natural route vocabularies.** Already covered by `route-transposition`, `dictionary-attack`, and `exhaustive-enumeration`.
- **Special zero, half-turn, equal, and antipodal residue cases.** The algebra is distributed across `permutation-cycles`, `dihedral-group`, `semiregular-common-cycle-criterion`, and `periodic-substitution-dp`. The observed beam failure belongs under power measurement, not a new cipher primitive.
- **Physical-group phase expansion.** Already covered by `reflection-product-cycle-lift`, `conjugacy-and-gauge`, `candidate-bank-lineage-and-cap-accounting`, and the partial-involution exact pages.

## Remaining evidence gaps

- The full-residue anchor theorem is strongly verified as software and finite-family algebra, but its disclosed implementation used one null. That is enough for correctness checks, not for a small permutation-test p-value.
- The translated-set solver has a complete scoped zero result for one application. Wider periods, different coordinate orders, and non-additive actions need separate runs; the article deliberately does not generalise that closure.
- The Hamming-radius invariant is exact, but a useful positive detector would need preregistered alignments and a matched-profile null across the full selected family.
- The partial-reflection equality result is software-only and conditional. It still needs fresh end-to-end controls when used inside a larger search, especially where the source proposal bank can miss the true partial map.
- Several long-running search notes report caps, shortlist loss, or failed control power rather than mathematical closure. Their lessons are already documented as validation principles; they are not promoted to verified cipher-family negatives.
- Source receipts are spread across a large ledger and many logs. A future provenance pass could add stable content-addressed manifests without changing the mathematical articles.

## Result

Four new articles were accepted. Each adds a distinct reusable theorem or exact reduction, gives its algebra and pseudocode, names its assumptions, and separates software correctness from unknown-text evidence. No cipher-family narrative, live text, or claimed solve was imported.
