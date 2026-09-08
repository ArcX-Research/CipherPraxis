# Research integration review — 2026-09-08

This is a maintainer record, not a public knowledge-base article. Public explanations use
general cryptographic models, synthetic examples and literature references. Source paths remain
archival evidence. No claim of global priority follows from a method being developed locally.

## Scope and source handling

The review used three existing checkouts:

| Alias | Local collection |
| --- | --- |
| `archive-a` | `/Users/nitrika/Projects/Mywork/ArcX-Research/Research/Cryptography/pk8-research` |
| `archive-b` | `/Users/nitrika/Projects/Mywork/ArcX-Research/Research/Cryptography/pk9pk10-research` |
| Unprefixed provenance / `analysis` in the inventory | `/Users/nitrika/Projects/Mywork/questlyst/Cryptanalysis` |

The third path resolves the user's abbreviated `/Projects/...` location to the existing
checkout. The working catalog initially contained 306 entries, including six files with
uncommitted work. Those edits were retained. Existing entries and earlier coverage records
were compared with source implementations and recent mathematical and experimental notes.

`RESEARCH_SOURCE_INDEX_2026-09-08.json` records the selected source files as found: 158 from
the first archive, 301 from the second and 2,211 from the analysis collection. It includes path,
byte length, SHA-256 and matching article citations. Installed environments, vendored source
trees, private input banks and generated experimental output are outside this source inventory.
The inventory is broader than the editorial reading: it does not assert that every indexed file
was read in full, executed or validated. It is not a historical execution receipt.

No private ciphertext, plaintext, hint or crib list was imported. Existing result receipts remain
historical observations under their recorded conditions. Expensive research jobs and model
benchmarks were not rerun. Public claims distinguish derivation, source inspection, exact finite
checks, measured recovery and unfinished designs.

## Canonical additions

The catalog now contains 317 entries. The 11 additions represent distinct models or procedures:

| New canonical id | Material incorporated | Why it warrants a separate entry |
| --- | --- | --- |
| `progressive-polyalphabetic-key` | Per-residue intercept/slope models; cached candidate streams and local score changes | A changing key schedule beyond a fixed periodic key |
| `circulant-hill` | Structured linear systems over two fields; reconstruction and invertibility checks | A structured Hill key with only one independent row |
| `linear-feedback-hill` | Feedback-placement cancellation and linear output-feedback recurrence screens | Feedback changes the observed sequence and initialization requirements |
| `morbit-and-pollux` | Morse-pair permutations and homophonic Morse-symbol maps | These are different key spaces from fractionated Morse |
| `rotor-machines` | Reciprocal conjugation, cached rotor maps, ring/window distinction and double stepping | A stateful machine family missing from the catalog |
| `independent-coordinate-alphabets` | Separate stream maps, map-qualified variables and gauge constraints | Independent bijections cannot be replaced by one shared alphabet |
| `hill-row-recovery` | Ranked decryption-row banks, finite-field null-space filters and cached scoring | A specific attack and optimization, separate from the Hill cipher definition |
| `joint-transposition-periodic-search` | Joint composition search, position phases and ragged-column conventions | Coupled key variables require a different search state from either layer alone |
| `block-word-boundary-dp` | Max-plus boundary transfers for fixed-length dictionary blocks | A block model and cached transfer distinct from variable-length word segmentation |
| `incremental-language-score` | Deduplicated affected windows, exact deltas, undo and Gray-code traversal | A reusable scoring optimization across several cipher families |
| `shared-prefix-neural-cache` | Immutable prompt sharing, branch reordering and coupled-stream cache checks | A specific memory/state optimization beyond neural candidate scoring |

Parent articles link to the additions. Search tags and families use the existing taxonomy where
appropriate. Standard constructions cite primary documentation or original literature where
available; locally derived optimizations are described without unsupported priority or speed claims.

## Merges and corrections

Five overlapping drafts were folded into existing articles before publication. No existing route
was removed:

| Material | Canonical article |
| --- | --- |
| Quadratic-pencil enumeration | `algebra/finite-field-linear-algebra` |
| Completion of observed coordinate assignments | `algebra/conjugacy-and-gauge` |
| Delayed velocity introduction in word-path search | `ciphers/piecewise-constant-velocity-disk-cipher` |
| Dual-trie running-key decoding and aggregated resets | `exact/word-lattice-segmentation` |
| Modular-potential union-find | `algebra/residue-class-graphs` |

Further material extends the existing treatment rather than creating another page:

- `linear-recurrences-lcg`: exact finite affine-order recovery from first differences over the
  two fields, CRT reconstruction and replay.
- `running-key-relation-test`: elimination using a verified third stream, with distinct literal,
  independent-map and shared-cycle assumptions.
- `label-free-collision-geometry`: curvature-indexed triple factors and the later four-case
  power result; exact arithmetic checks do not erase the recorded recovery failures.
- `mnemonic-wordlist-encoding`: using dictionary words as a key is separate from a full
  checksum-constrained mnemonic encoding.
- `fractionated-morse`: language scoring in the Morse representation and decoding controls.
- Existing cost-projected bounds, exact-optimization admission and valid-model null articles
  retain the overlapping results already present in the working catalog.

The published Gray-code traversal explicitly scores the initial all-zero assignment; the
reviewed loop starting at the first flip omitted that candidate. Row-bank shortlists, beam caps,
neural scores and restricted ring searches are identified as heuristic or scoped. The delayed
velocity proposal remains a design until the implementation passes its own comparisons.

Research-specific summaries and the public evidence guide were generalized. Bibliographic
references appear before archival records, which can be expanded independently. Provenance
remains available for audit without defining the public concepts in terms of research projects.

## Verification and editorial boundaries

The strict catalog check verifies unique ids, references, block ordering, required pseudocode,
forbidden public tokens and source metadata. The provenance check resolves all three research
collections plus this site. A duplicate review compares titles, summaries and substantial blocks,
then assesses conceptual overlap against the existing parent articles. The existing pair
`key-cancellation` / `g-key-cancellation` is an intentional full article and short glossary entry.

`scripts/check_research_algorithms.py` uses independent finite calculations for circulant
inversion and rank, Hill row filtering, weighted modular union-find, incremental score changes
and Gray-code enumeration, affine recurrence order, triple curvature, quadratic-pencil counts,
observed-coordinate completions, and cached word-boundary transfers with complete tied-path
comparisons. These checks use synthetic inputs and do not establish
general recovery power or validate every source implementation.

The Rust suite covers the content renderer, search and interactive cipher/statistics core.
The complete check also covers native and WebAssembly linting, mathematical spans, list
rendering and cited-path existence. Browser review covers the overview, article mathematics,
pseudocode navigation, search and evidence disclosures. The release build remains a local
artifact; this review does not record a deployment or independent peer review.
