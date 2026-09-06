# Cipher Praxis content-gap inventory

Internal maintainer note; this is not a public knowledge-base entry.

Current-state addendum (2026-09-05): later deep, cross-cipher, literature, and
math-validation passes expanded the catalog from the 228-entry snapshot below
to 253 entries. A 2026-09-06 review of LOG634–638 then added four distinct
validation, engineering, and exact-enumeration abstractions, bringing the
catalog to 257 entries. Their decisions are recorded in
`DEEP_INVENTORY_2026-09-05.md`, `CROSSCIPHER_INVENTORY_2026-09-05.md`,
`LITERATURE_AND_MODEL_INVENTORY_2026-09-05.md`,
`EARLY_LITERATURE_INVENTORY_2026-09-05.md`, and
`MATH_VALIDATION_INVENTORY_2026-09-05.md`; the live delta is in
`LIVE_DELTA_INVENTORY_2026-09-06.md`. The tables below remain the audit snapshot
for the earlier 22-entry pass rather than being silently rewritten.

Snapshot: 2026-09-05, after a strict parse of 228 TOML entries. This inventory compares current entry IDs with reusable methods that have stable derivations, controls, software checks, or clearly labelled designs in `../Cryptanalysis/withmath`. It is not a claim that the source corpus or the external literature has been exhaustively surveyed.

## Coverage after this pass

| Reusable technique | Current coverage | Evidence anchors | Assessment |
|---|---|---|---|
| First-, reverse-, ciphertext-, and symmetric second-order coordinate feedback | `plaintext-coordinate-feedback`, `right-to-left-plaintext-feedback`, `ciphertext-coordinate-feedback`, `symmetric-second-order-plaintext-feedback` | LOG554, LOG623, LOG624/626, LOG630/632/633 | Covered, including direction aliases, zero-divisor strata, boundaries, independent forwards, and fresh exact-text controls. |
| Gauges, raw cells, observable classes, and latent labels | `conjugacy-and-gauge`, `group-actions-on-alphabets`, `isomorph-analysis`, `feedback-state-gauges-and-cells`, `latent-label-identifiability`, `observable-function-fingerprinting` | LOG623, LOG624, LOG630 | Covered, including exact small-domain fingerprints, larger-domain separation witnesses, and the limit of finite probes. |
| Structural recovery versus exact text and key labels | `parameter-versus-plaintext-recovery`, `endpoint-specific-control-contracts`, `tautological-gate-rule` | LOG632/633 | Covered, including a retained endpoint failure followed by a separately frozen fresh-panel pass. |
| Character, lexical, and neural candidate scoring | `quadgram-log-likelihood`, `semi-markov-word-model`, `local-language-model-prior`, `n-gram-language-scoring`, `word-lattice-segmentation`, `combined-character-word-models`, `neural-language-model-decipherment`, `lexical-reranking-and-oov-bias` | LOG429/430, boundary and semantic audits; Kambhatla et al. 2018; Hauer et al. 2014 | Covered, with local neural/combined power limitations retained. |
| Exact assignment, dynamic programming, and certified pruning | `exact-dynamic-programming`, `viterbi-decoding`, `periodic-substitution-dp`, `bound-benchmarks-and-certificates`, `maximum-weight-bipartite-assignment`, `partial-permutation-completion-bounds` | LOG573, LOG578/579, assignment/Hungarian and branch-and-bound audits | Covered at the theorem and software-check level. Whether the stronger completion bound saves time in a hot loop remains workload-dependent. |
| Local permutation diagnostics and bounded shells | `hill-climbing`, `polish-basin-measurement`, `identity-plus-one-swap-neighborhood`, `bounded-permutation-shells` | complete single-swap audits; LOG573, LOG603, and LOG633 | Radius one is verified, including a fresh panel of complete neighbourhoods. The higher-radius component-shell construction is documented, but remains `UNTESTED` because LOG603 is design-only. |
| Adaptive validation and fresh evaluation | `held-out-gates`, `selection-overfit-check`, `selection-overfit-diagnostic`, `fresh-holdout-after-selector-change` | LOG430 plus methodology literature | Covered. |
| Candidate lineage, semantic caps, and proposal voting | `candidate-bank-lineage-and-cap-accounting`, `unique-proposal-voting`, `k-best-lists`, `conjugacy-and-gauge` | LOG589, LOG605, LOG607, LOG615 | The accounting and variable-denominator arithmetic are covered. Fresh end-to-end recovery power for every voter count remains open. |
| Acquisition, provenance, strict replay, and mutation refusal | `reproducibility-receipts`, `seal-and-audit-protocol`, `kat-harness`, `isolated-acquisition-solver-handoff` | LOG631 | Covered at the protocol level. |

The 22 new IDs in this extended pass are:

- Algebra: `feedback-state-gauges-and-cells`, `latent-label-identifiability`.
- Ciphers: `plaintext-coordinate-feedback`, `right-to-left-plaintext-feedback`, `ciphertext-coordinate-feedback`, `symmetric-second-order-plaintext-feedback`.
- Cryptanalysis: `parameter-versus-plaintext-recovery`, `neural-language-model-decipherment`, `observable-function-fingerprinting`.
- Search and exact: `identity-plus-one-swap-neighborhood`, `bounded-permutation-shells`, `unique-proposal-voting`, `word-lattice-segmentation`, `maximum-weight-bipartite-assignment`, `partial-permutation-completion-bounds`.
- Statistics: `n-gram-language-scoring`, `combined-character-word-models`, `lexical-reranking-and-oov-bias`.
- Validation and engineering: `endpoint-specific-control-contracts`, `fresh-holdout-after-selector-change`, `isolated-acquisition-solver-handoff`, `candidate-bank-lineage-and-cap-accounting`.

The follow-up gap pass added four implemented-method pages—`candidate-bank-lineage-and-cap-accounting`, `unique-proposal-voting`, `observable-function-fingerprinting`, and `partial-permutation-completion-bounds`—plus the design-only `bounded-permutation-shells` page. These close documentation gaps, not the validation limits stated on the pages.

## Remaining validation and power gaps

These are evidence gaps, not requests for duplicate method pages:

1. **Bounded-shell implementation and recovery power** — `bounded-permutation-shells` defines the accepted family, duplicate handling, distance checks, and completeness contract. The higher-radius generator, independent accepted-set KAT, finite-budget control panel, and fresh end-to-end power gate have not been run. Keep the entry `UNTESTED`; a miss from a capped future run would not close the family.
2. **Contamination-bounded word/subword semantic reranking** — `neural-language-model-decipherment` already states the literature method, tokenisation contract, contamination boundary, and failed local gates. The source corpus still has no passing uncontaminated local semantic gate. Keep this as a power gap until one frozen model and a genuinely external or decontaminated test bank pass together.
3. **Variable-voter recovery power** — `unique-proposal-voting` has exact arithmetic, duplicate-refusal, and interface checks, but those do not show that every permitted voter count retains the correct candidate in a blind language task. A fresh panel must test the unchanged proposal generator and all admitted voter counts.
4. **Hot-loop value of stronger completion bounds** — `partial-permutation-completion-bounds` records exact formulas and reconstructed witnesses. A workload audit found only modest extra pruning for one static use, so a production port still needs an overhead-versus-pruning test and full accepted-set equality.
5. **Scope of observable fingerprints** — `observable-function-fingerprinting` proves equality only on complete finite domains and proves non-equality from a differing probe. Matching fixed probes on a larger domain remain unresolved unless a symbolic proof or full exhaustion is added.

## Topics that are not gaps

Do not create duplicate pages for these without a genuinely different abstraction:

- Branch and bound, admissible majorants, tie counts, and incomplete sentinels are already in `bound-benchmarks-and-certificates`.
- Strict parser mutation matrices are already in `kat-harness`, with the acquisition boundary in `isolated-acquisition-solver-handoff`.
- Atomic sealing, no-clobber receipts, and source hashes are already split across `seal-and-audit-protocol`, `ledger-reservations`, and `reproducibility-receipts`.
- General Viterbi decoding is in `viterbi-decoding`; unspaced lexical lattices now have the narrower `word-lattice-segmentation` entry.
- Gauge-orbit deduplication is in `conjugacy-and-gauge` and `group-actions-on-alphabets`; semantic caps and full witness survival are now covered separately by `candidate-bank-lineage-and-cap-accounting`.

The next evidence sequence is: implement the target-absent bounded-shell set-equality tests, measure its finite-budget controls, test variable-voter recovery on fresh data, and only then evaluate an uncontaminated word/subword reranker. None of those pending experiments justifies another near-duplicate page today.
