# Early-script and literature inventory — 2026-09-05

Internal maintainer note; this is not a public knowledge-base entry.

## Scope

This pass compared the early `scripts/ck_*.py` and `scripts/poem_*.py` probes,
the external-family audit, and the current catalog. Definitions were checked
against the American Cryptogram Association's public cipher-type materials.
Literature coverage is kept separate from solver power: a correct transform
does not imply that a blind search has passed controls.

## Added entries

| Entry | Internal trail | Status boundary |
| --- | --- | --- |
| `gronsfeld` | Numeric-sequence probes in `scripts/ck_more.py` | `UNTESTED`: the named restricted Vigenere family had no dedicated control panel. |
| `baconian` | Binary-class and phase extractor in `scripts/poem_final.py` | `UNTESTED`: the extractor exists, but no planted cover/distractor panel qualifies it. |
| `ragbaby` | The 24-letter alias appeared in the external-family audit. | `UNTESTED`: literature-backed mechanics, no dedicated solver result. |
| `monome-dinome` | The merged-letter implementation appeared in the external-family audit. | `UNTESTED`: literature-backed prefix code, no dedicated solver result. |
| `digrafid` | The all-family audit recorded that execution stopped before this family. | `UNTESTED`: preserving the transform explicitly avoids turning a timeout into a negative. |

## Rejected or already covered

- ADFGX-style coordinate transposition is already captured by
  `coordinate-columnar-fractionation`; a second page would repeat the same
  coordinate stream and columnar middle layer.
- Slidefair and Portax are already combined in `digraphic-slide`.
- Seriated Playfair is described as a Playfair variant in the existing
  digraphic material; the incomplete early solver is not evidence for a new
  verified page.
- Gromark and Condi already have full method, attack, and control pages.
- Direct arithmetic sequences such as Fibonacci, primes, and decimal
  constants are candidate key sources, not separate cipher algorithms.

## Source rule

All five pages use original examples derived from their definitions. Public
reference links point to the ACA pages used to check conventions. Internal
provenance says only where the project encountered or implemented the family;
it does not upgrade an inventory mention into a solver result.
