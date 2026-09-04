# Authoring guide for Cipher Praxis content

Cipher Praxis — A Dilate Cryptography Knowledge Base — is a **public, general-cryptography**
reference. The repository around it (`../poemanalysis/`, its `withmath/` directory) is only the internal evidence
corpus. Read `../ARCHITECTURE.md` §1 and §4 first, then the exemplar `ciphers/vigenere.toml`.

## Hard rules (enforced by the build lint and by review)

1. Public fields (`title`, `subtitle`, `summary`, `status_note`, `family`, `tags`, block `title`
   and `body`) contain general cryptography only. Forbidden, case-insensitive, word-bounded:
   `kryptos`, `ctf`, `sanborn`, `langley`, `cia`, `pk0`…`pk10`, `pk89`, `pk98`, `pk8910`,
   `k1`…`k4` (as puzzle labels), `leaderboard`, `submission`, `poem`, `paradigm`.
   Write "a 153-letter ciphertext", "the 144-letter text", "two related ciphertexts", "the
   second text", "text A / text B" instead of puzzle names.
2. Never reproduce a live ciphertext, plaintext, hint, story vocabulary, or crib list from the
   corpus. Examples are generic and reproducible: state your own plaintext and key, derive the
   ciphertext by hand or by the definition, and show the check.
3. Every number quoted in public text (pass rates, z-scores, power, throughput, counts) must come
   from a file named in `[[provenance]]` with `path` (repository-relative) and, when it is a log
   or ledger entry, `ref` (for example `LOG403` or `logs/152_crosscycle_full.log`). Numbers not
   backed by a receipt are removed, not rounded.
4. Status vocabulary: `VERIFIED`, `PROMISING`, `CLOSED`, `POWER-LIMITED`, `INCONCLUSIVE`,
   `UNTESTED`. The `status_note` says in one or two sentences what the status rests on, in
   general terms ("planted controls 10/10 at 153 letters; blind unknown alphabet fails 0/24").
5. `related` lists ids of other entries. Use the canonical ids below for cross-domain links; the
   build warns on ids that do not exist, so prefer canonical ids and the ids you create.
6. One TOML file per entry at `content/<section>/<id>.toml`; the file name equals the id.
   Only write inside the directories assigned to you.
7. Block `kind` values: `definition`, `equations`, `variants`, `assumptions`, `attack`,
   `pseudocode`, `complexity`, `failure_modes`, `controls`, `example`, `notes`, `history`. Use the
   ones that apply, in that order. Every method entry carries a `pseudocode` block: one fenced
   code block (```` ```text ````) of 8–30 lines of language-neutral pseudocode that restates the
   entry's own definition or procedure (ENCRYPT/DECRYPT for ciphers, the estimator for statistics,
   the search loop for algorithms, the protocol steps for validation and engineering practices).
   Pseudocode introduces no numbers or claims that the surrounding blocks do not already make. Bodies are Markdown; math is LaTeX in `$…$` / `$$…$$` (keep to a
   standard subset: fractions, subscripts, superscripts, sums, `\bmod`, `\pmod`, `\equiv`,
   `\mathbb{Z}`, `\operatorname{}`, Greek letters, `\le`, `\ge`, `\ne`, `\cdot`, `\times`).
   Internal links use site routes: `[Beaufort](/ciphers/beaufort)`.
   Two fence languages get special rendering. ```` ```trace ```` is for worked-example
   walkthroughs: one row per line, a label (a word followed by two spaces, or a short `A:` prefix),
   then the tokens. Columns are separated by two or more spaces; a bracketed expression such as
   `13+(23 mod 13)=23` stays one cell, and a run of eight or more capitals is split per letter.
   Rows of equal length share one aligned grid; shorter or longer rows that carry separators
   (`+`, `=`, `/`, `|`, `...`) or long tokens flow inline. Rows labelled `cipher`, `outer`,
   `output`, `c_i` or `sigma` are highlighted. ```` ```tree ```` is for directory listings: a
   path, two spaces, a description.

8. Provenance `kind`: `solver`, `library`, `audit`, `design`, `note`, `log`, `script`, `data`,
   `ledger`. Provenance `note` may use internal file names but should still avoid quoting
   ciphertext or hints.
9. Dates: `updated = "2026-09-04"`.

## Canonical ids (use these spellings for cross-links)

ciphers: `caesar-shift`, `affine-cipher`, `keyword-mixed-alphabet`, `vigenere`, `beaufort`,
`variant-beaufort`, `porta`, `quagmire-i`, `quagmire-ii`, `quagmire-iii`, `quagmire-iv`,
`gromark`, `condi`, `alberti-progressive`, `chaocipher`, `autokey`, `running-key`,
`compound-periodic-key`, `hill-cipher`, `playfair`, `two-square-four-square`, `bifid`, `trifid`,
`columnar-transposition`, `route-transposition`, `rail-fence`, `myszkowski`,
`sandwich-construction`, `interleaved-streams`, `compressocrat`, `additive-stream-wheels`,
`rsa-textbook`, `mnemonic-wordlist-encoding`, `one-time-pad`, `book-cipher`, `nihilist`,
`polybius-square`, `digraphic-slide`.

algebra: `modular-arithmetic-z26`, `units-mod-26`, `modular-inverse-and-crt`,
`affine-maps-mod-26`, `permutation-cycles`, `conjugacy-and-gauge`, `dihedral-group`,
`finite-field-linear-algebra`, `linear-recurrences-lcg`, `difference-operators`,
`all-different-constraints`, `matrix-invertibility-mod-26`, `lcm-of-periods`, `group-actions-on-alphabets`.

cryptanalysis: `kasiski-examination`, `exact-period-alignment`, `coset-shift-recovery`,
`crib-dragging`, `crib-driven-alphabet-recovery`, `key-cancellation`,
`autokey-difference-periodicity`, `mixture-inference-under-transposition`,
`order-free-width-statistic`, `phrase-keyed-alphabet-attack`, `segment-chain-analysis`,
`shared-cycle-attack`, `change-point-zone-detection`, `family-identification`,
`running-key-relation-test`, `multiple-rounds-analysis`, `transposition-width-recovery`,
`frequency-analysis`, `isomorph-analysis`, `assumption-review-protocol`.

statistics: `index-of-coincidence`, `coincidence-lag-profile`, `chi-squared-fit`,
`quadgram-log-likelihood`, `bayesian-evidence-model-selection`, `mahalanobis-family-distance`,
`shuffled-null-z-score`, `family-wise-null`, `within-coset-null`, `permutation-test`,
`power-analysis`, `selection-overfit-diagnostic`, `semi-markov-word-model`,
`local-language-model-prior`, `english-gate-calibration`, `estimator-standard-error`.

search: `simulated-annealing`, `parallel-tempering`, `hill-climbing`, `beam-search`,
`k-best-lists`, `viterbi-decoding`, `expectation-maximisation`, `coordinate-descent`,
`meet-in-the-middle`, `rank-sharding`, `corpus-bank-screen`, `dictionary-attack`,
`polish-basin-measurement`.

exact: `constraint-satisfaction-forward-checking`, `cp-sat-and-smt-solvers`,
`exact-dynamic-programming`, `exhaustive-enumeration`, `all-different-enumeration`,
`exact-chain-oracle`, `bound-benchmarks-and-certificates`, `known-answer-tests`,
`word-prefix-csp`, `periodic-substitution-dp`.

validation: `planted-controls`, `control-first-policy`, `target-absent-controls`,
`blind-controls`, `power-measurement`, `tautological-gate-rule`, `valid-model-nulls`,
`seal-and-audit-protocol`, `reproducibility-receipts`, `ledger-reservations`,
`correction-and-retraction`, `selection-overfit-check`, `throughput-fail`, `held-out-gates`,
`gauge-defect-invalidation`.

engineering: `numpy-instrument-pattern`, `compiled-hot-cores`, `kat-harness`, `seed-determinism`,
`drainability-and-throughput`.

labs (owned by the app author): `vigenere-lab`, `quagmire-lab`, `coincidence-lab`,
`affine-lab`, `columnar-lab`, `hill-lab`, `bifid-lab`, `crt-lab`, `dihedral-lab`, `null-lab`.

glossary ids are prefixed `g-` (for example `g-coset`, `g-quadgram`, `g-planted-control`).
