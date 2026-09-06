# Cryptanalysis-to-catalog reconciliation — 2026-09-06

Internal maintainer note. This file is not a public knowledge-base entry.

## Scope and method

This pass began with 306 catalog entries. It re-enumerated the sibling
`../Cryptanalysis` evidence corpus: 449 Markdown notes under `withmath/docs/`
and 1,478 Python, C/C++, Rust, shell, and header source files across the
repository. It compared current entry ids, titles, summaries, blocks, and
1,217 provenance records (617 distinct paths), rather than assuming that a
similar filename meant equivalent coverage.

The earlier inventory notes already record complete bounded reviews of the
early scripts and literature, all cipher-method and cross-cipher notes at
their cutoffs, and the mathematical, language, validation, and exact-solver
material through LOG638. This pass therefore used those decisions as its
baseline and performed a fresh semantic review of every numbered note from
LOG639 through LOG725. There were 85 such notes. After the changes below, 62
are cited directly by a public article; the other 23 are accounted for in the
grouped table below.

Public content remains general cryptography. No puzzle narrative, live text,
hint, crib, or candidate was copied into the catalog.

## Material added to existing articles

No new public page was needed. The reusable ideas fit existing articles more
cleanly than near-duplicate entries:

| Source | Article updated | Reusable knowledge retained |
| --- | --- | --- |
| `PK89_PLAINALPHA680_DESIGN.md` | `phrase-keyed-alphabet-attack` | Derive a finite alphabet bank from a separately recovered related text using frozen serializations and completion orders; preserve every physical origin; admit only complete, independently replayed upstream endpoints; search the downstream periodic key exactly when feasible. |
| `ZONE25_715_FULL_SELECTOR.md` | `proposal-bank-recall-audit` | Counterfactual truth insertion separates score quality from proposal access. A truth that would rank first but never enters the endpoint bank cannot be recovered by a later reranker. |
| `PK89_SHAREDMAP721.md` | `cross-residue-collision-alignment` | Screen literal arbitrary maps shared across texts with centered cross-collisions and an exact profiled monogram-assignment penalty. Reselect every phase and residue alias inside one-source relabeling nulls. The recorded short-text selector is power-limited. |
| `PK10_BIGRAM722.md` | `label-free-collision-geometry` | Lift the squared-distance collision identity from monograms to ordered adjacent-pair distributions. This can retain order with uniform monograms, but circulant pair laws create aliases and the implementation remains design-only. |
| `PK10_ROWEQ725.md` | `coincidence-lag-profile` | Scan all fitting widths, positive row gaps, and starts for aligned equalities, then calibrate the selected maximum with complete fixed-multiset permutations. A small fixed-tuple tail can be null-like after global selection. |
| `QUANTUM_SEARCH_TRIAGE.md` | `quantum-search-triage` | Trace the distinction between physical quantum protocols, Shor-type number-theoretic attacks, Grover oracle search, and classical quantum-inspired heuristics to an internal literature note and the cited primary papers. |

The changes add variants, equations, limitations, controls, and traceable
provenance to established articles. They do not broaden a scoped negative or
inherit a `VERIFIED` status from an unimplemented extension.

## Late notes not cited directly

The remaining 23 late notes do not justify additional public pages:

| Notes | Count | Decision |
| --- | ---: | --- |
| `PK89_DUAL642_GATE.md`, `PK89_DUAL645_GATE.md`, both LOG653 controller/preflight notes, `PK10_INTEGRATED654_CONTROLS.md`, `PK10_INTEGRATED673.md`, `PK89_PIPELINE675_LIVE.md`, `PK89_PIPELINE677_REPAIR.md` | 8 | Transaction and controller stages already covered by `commit-before-reveal-evaluation`, `isolated-acquisition-solver-handoff`, `reproducibility-receipts`, and `transitive-process-census`. They are successive release implementations, not new cryptographic algorithms. |
| `PK10_CONTEXTRELABEL643_CONTROLS.md`, `PK10_LEXBAND650_SCORER.md`, `PK10_LEXBAND651_INDEPENDENT_AUDIT.md` | 3 | Development or pre-release records superseded by the cited fresh controls and audits behind `contextual-rare-symbol-relabeling`, `lexical-band-contextual-reranking`, and `score-gap-ambiguity-bands`. |
| `PK89_WINDOWS644_AUDIT.md`, `PK89_FRONTIER_AFTER647.md`, `FOURCLOCK696_LIVE.md` | 3 | Exclusion metadata, a frontier review, and one live result. Their reusable rules or construction are already cited from the design/audit pages; the remaining text is project history. |
| `PK89_UNTIEDCYCLE655_THEORY.md`, `PK89_UNTIEDCYCLE656_CONTROLS.md`, `PK9_ROWCYCLE658_PROBE.md`, `PK9_RESIDUECRT661_POWER.md` | 4 | Precursors to the cited LOG659/660/663/664 work and the existing common-cycle, CRT-join, proposal-recall, and exact-completion articles. |
| Both LOG693 QIV-chain notes, `TURNGRILLE698_REX_PROTOTYPE.md`, `PK89_T9GENERATOR701.md` | 4 | A failed all-cell gate, a superseded grille prototype, and an early-beam power result. Their general lessons are already in `constraint-activation-timing`, `turning-grille`, `sparse-markov-observation-mixing`, and `proposal-bank-recall-audit`. |
| `PK9_ALPHAPERM716.md` | 1 | The source is unfinished and its available transcripts include a stale-source run and a failed injectivity implementation. Its stable primitives—finite skeleton tiling, word-prefix CSP, CRT feasibility, and all-different completion—already have articles. No result from this source was promoted. |

LOG723 and LOG724 are completed power-failure pilots recorded only in the
coordination ledger; their prefix-loss lesson is already covered by
`beam-search`, `constraint-activation-timing`, and
`proposal-bank-recall-audit`. LOG726 had a reservation and non-terminal pilot
artifacts but no completed audit or result note at this cutoff. Its proposed
WFSA, admissible suffix bound, injection CSP, and matching propagation are
compositions of existing exact-solver articles, so no unfinished page was
added. LOG727 was reserved after the reviewed LOG725 cutoff as a crash-safe
continuation of an interrupted frozen run; its checkpoint/replay rules are
already represented by the reproducibility and immutable-artifact articles,
and any later mathematical result needs a fresh delta review.

## Duplicate audit

The strict parser reports zero duplicate ids. Normalized summaries also have
zero exact duplicates. A pairwise token-overlap scan covered all 230
non-glossary, non-lab, non-reference method articles. Only three pairs crossed
the conservative review threshold:

- `quagmire-i` and `quagmire-ii`: different placement of the mixed alphabet;
- `plaintext-coordinate-feedback` and
  `symmetric-second-order-plaintext-feedback`: first-order versus two-sided
  second-order state equations;
- `hill-cipher` and `matrix-invertibility-mod-26`: a cipher construction
  versus the algebraic invertibility test it uses.

Manual review also retained the deliberately separated diagnosis/protocol
pairs `selection-overfit-diagnostic` / `selection-overfit-check`,
`power-analysis` / `power-measurement`, and `known-answer-tests` /
`kat-harness`. Their endpoints and procedures differ. No true semantic
duplicate was found, so deleting an article would have discarded distinct
technical content or broken a stable label. No public entry was removed.

## Repository-name repair

Authoring, architecture, inventory, and evidence-panel text now identify the
evidence corpus as `Cryptanalysis`, matching the renamed sibling repository.
The old `poemanalysis` label is no longer shown as the corpus root.

## Verification

- `make check`: passed formatting, shell/Python syntax, native and WebAssembly
  Clippy with warnings denied, 50 tests, strict content validation, math-span
  and list-marker checks, and the WebAssembly type check.
- Strict content result: 306 files, 306 entries, zero errors, zero warnings.
- `make provenance`: 306 content files, 1,217 provenance records, zero missing
  paths. The pass repaired 208 stale path names affecting 553 citations after
  the evidence repository moved notes and scripts into subdirectories.
- `make build`: completed the optimized WebAssembly release and generated the
  static site bundle.
- `git diff --check`: passed.
- Repeated duplicate scan: zero duplicate ids, zero duplicate normalized
  summaries, and no true semantic duplicate after manual review of the three
  high-overlap pairs.
