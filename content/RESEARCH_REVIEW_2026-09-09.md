# Research integration review — 2026-09-09

Maintainer record, not a public knowledge-base article. Public explanations use general
cryptographic models, synthetic examples and literature references; source paths remain
archival evidence. No claim of priority follows from a method being developed locally.

## Scope and source handling

Two working directories beside the two archives already aliased were imported as evidence:
the solution record of a 153-letter corpus text (`pk8-solution/`: a self-contained
re-derivation and its README, plus an unchanged copy of the worker, scheduler, crib bank,
control scripts, control logs and hit record) and the attack workspace for a 144-letter
corpus text (`pk9-attack/`: periodic-coincidence profile script, crib-solve summaries and
log, three crib-free solvers with one planted control and its three recorded runs).

Every file under `pk8-solution/original-code/` is byte-identical to its original in the
first archive (`archive-a/`, including the hit record
`mixed-cribs-mixed-cribs-core-pk8/1055.jsonl` and the scheduler log), so those files are
cited at their originals. A third alias, `archive-c/`, resolves at the root of the local
research collection that contains both new directories
(`scripts/check_provenance.py --archive-c`, default
`~/Projects/Mywork/ArcX-Research/Research/Cryptography`) and is used only for the files
without an archived original: the re-derivation, its README and the attack workspace. The
alias tables in `AUTHORING.md`, `ARCHITECTURE.md`, `README.md` and
`references/internal-evidence-corpus` were extended. The dated source index of 2026-09-08
was not regenerated. Avoiding the alias entirely would require moving those files into an
aliased checkout or copying them into this repository; the latter would import ciphertext,
plaintext and key material, so neither was done.

No ciphertext, plaintext, crib bank content, key words or raw key residues were imported into
public fields; provenance notes name the files that hold them. Empirical numbers in public text
are quoted from the cited files; the periodic-coincidence z-scores were reproduced by running
the cited script on 2026-09-09, since no saved log of its output exists in the workspace.

## Canonical additions

| New canonical id | Section | Material incorporated | Why it warrants a separate entry |
| --- | --- | --- | --- |
| `sum-of-periodic-keys` | ciphers | Several Vigenère layers in one keyed alphabet; combined period beyond the text; degree 18 for periods 4, 5, 6, 7; planted controls; the solved 153-letter text | `compound-periodic-key` covers two keys solved at their combined period; here the period outruns the text and the solve is linear-algebraic |
| `linear-keystream-crib-solve` | cryptanalysis | Crib slide with exact GF(2)/GF(13) solve, keystream-uniqueness test, CRT lift, keystream recurrence, scheduler, controls, negatives on the 144-letter text | The crib fixes the degree, not the period; a different state space and stopping rule from `crib-dragging` |

## Merges

Four drafts were folded into the existing canonical article for their concept and removed;
no existing route was renamed or removed:

| Draft (removed) | Folded into | What was added there |
| --- | --- | --- |
| rank and gauge of a periodic-key sum | `algebra/lcm-of-periods` | Equations block "Rank and gauge of a sum of several periodic keys" (design matrix, rank as the cyclotomic count, nullity, kernel generators, keyword reading over the orbit), attack step, second pseudocode block, controls, worked examples, provenance |
| keyword recovery over the gauge orbit | `algebra/lcm-of-periods` (pointer bullet in `search/dictionary-attack`, pointer sentence in `algebra/conjugacy-and-gauge`) | The orbit-and-sign enumeration against word lists, its unique hit on the solved text, and the labelling caveat |
| coincidence excess at component periods | `statistics/coincidence-lag-profile` | Equations block "Component periods of a summed key", a variants bullet, an attack step, a failure mode, the recorded z-scores and negatives under controls, a mechanism example, provenance |
| component cancellation by lag differences | `algebra/difference-operators` | A "What cancels" bullet with the pair counts, a staged-solve attack step with the information count, a second pseudocode block, complexity, the failed 144-letter control under controls and failure modes, a worked pair table, provenance |

Cross-references were added to the `related` lists of `compound-periodic-key` (plus one variants
bullet), `vigenere`, `quagmire-iii`, `lcm-of-periods`, `conjugacy-and-gauge`,
`finite-field-linear-algebra`, `modular-inverse-and-crt`, `difference-operators`,
`crib-dragging`, `coincidence-lag-profile`, `dictionary-attack`, `g-gauge-freedom` and
`g-period`.

## Evidence boundaries

- The cipher and crib-solve entries are `VERIFIED` on the strength of five planted controls, the
  scheduler's controls-first assertion and the verified solve of one 153-letter text.
- The rank and gauge count in `lcm-of-periods` is a derivation with exact finite checks; the
  keyword reading is an exact enumeration on one text, not a recovery rate.
- The component-period profile in `coincidence-lag-profile` is a screening lead: random-letter
  null, no planted library for the family, multiplicity over 39 periods; the 144-letter text
  is not solved.
- The staged component cancellation in `difference-operators` is power-limited: its identity
  is exact, but the one recorded 144-letter control was not recovered by any of the three
  procedures; the status notes of both existing articles say so.

## Verification

`make content` (strict catalog check): 319 entries, 0 errors, 0 warnings. Math-span and
list-marker lints: 0 findings. `scripts/check_provenance.py`: 1,355 records, 0 missing across
the corpus, both archives, the new collection root and this repository. The web build and the
Rust test suite were not run for this import; the local watching dev server rebuilt `dist/`
from the changed content on its own.
