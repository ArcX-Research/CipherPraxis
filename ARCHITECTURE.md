# Cipher Praxis — A Dilate Cryptography Knowledge Base

Architecture and content-schema note. Read this before adding content or code.

## 1. Purpose and scope

Cipher Praxis is a public, general-cryptography knowledge base. It preserves and explains the
classical-cipher models, algebra, cryptanalysis techniques, statistical instruments, search and
exact solvers, validation protocols, and engineering practice supported by standard literature
and the local cryptanalytic research archives.

The repository is the **evidence corpus**, not the subject. Public content is written as general
cryptography research:

- No puzzle names, puzzle numbering, contest names, ciphertexts, plaintexts, hints, or operational
  puzzle detail appear in any public field. A build-time lint (`build.rs`) rejects forbidden
  tokens in public fields.
- Internal provenance (`[[provenance]]`) may cite project files, log ids, and audit notes so that
  every claim is traceable. The Evidence panel leads with literature and puts archival paths
  inside a "Supporting research records" disclosure. Provenance is exempt from the public-text
  lint (file names may contain internal identifiers).
- Results are reported as they were recorded: planted-control pass rates, power numbers, null
  distributions, throughput. Nothing is invented; missing evidence is `UNTESTED`.

## 2. Information architecture (public)

| Route | Section id | What it holds |
| --- | --- | --- |
| `/` | — | Overview: what the knowledge base is, how to read statuses, entry points |
| `/ciphers` | `ciphers` | Cipher Systems: families and individual cipher models |
| `/algebra` | `algebra` | Mathematical Foundations: the algebra and number theory used by the models and solvers |
| `/cryptanalysis` | `cryptanalysis` | Cryptanalysis (attack strategies) |
| `/statistics` | `statistics` | Statistical Analysis (estimators, nulls, evidence) |
| `/search` | `search` | Search Methods: heuristic search and optimization |
| `/exact` | `exact` | Exact Solvers: CSP, CP-SAT/SMT, DP, exhaustive enumeration |
| `/validation` | `validation` | Validation (planted controls, power, audits, receipts) |
| `/engineering` | `engineering` | Solver Engineering (cores, harnesses, reproducibility) |
| `/labs` | `labs` | Interactive cipher and analysis labs |
| `/glossary` | `glossary` | Terms |
| `/references` | `references` | Sources: literature and the internal evidence corpus |
| `/<section>/<id>` | — | Entry page for any record |

Every method page follows the same block order where applicable: definition, equations, variants,
assumptions & invariants, attack strategy, optimized pseudocode, complexity, failure modes, controls, reproducible generic
example, notes. Blocks are optional but the *kind* vocabulary is fixed (see §4).

## 3. Stack decision

Checked tools: `rustc`/`cargo` 1.95 with the `wasm32-unknown-unknown` target installed,
`wasm-bindgen` CLI 0.2.127, `wasm-pack` 0.13.1, Node 22, Bun; `trunk` and `wasm-opt` are not
installed and crates.io is reachable.

Decision: **Rust + Leptos 0.8 (client-side rendering) compiled to WebAssembly**, built with
`cargo build --target wasm32-unknown-unknown` and the installed `wasm-bindgen` CLI (pinned to the
same version in `Cargo.toml`). No Trunk dependency: one shell script builds, one serves. Rationale:

- The dependency lockfile and version checks constrain the build inputs. Byte-for-byte
  reproducibility still needs a recorded comparison under the same toolchain, environment,
  content, and optional optimization settings.
- Leptos gives fine-grained reactivity, a typed router, and small binaries; the whole UI, the
  search index, the Markdown+math renderer and the cipher labs run in WASM.
- The cipher and statistics code lives in `crates/core/src/crypto` with unit tests
  that run natively (`cargo test`), independent of the web layer.

Rendering pipeline: content TOML → `build.rs` (validate, lint, bundle to JSON) → `include_str!` →
`serde_json` at startup → in-memory indexes → Leptos views. Markdown is rendered with
`pulldown-cmark` (math extension enabled); `$…$`/`$$…$$` spans are converted to MathML with
`pulldown-latex` and rendered natively by the browser (no external JS, no KaTeX).

## 4. Content schema (`content/<section>/<id>.toml`)

One TOML file per entry. Fields:

```toml
id = "vigenere"                 # slug, unique across ALL sections; the URL is /<section>/<id>
section = "ciphers"             # ciphers | algebra | cryptanalysis | statistics | search | exact
                                # | validation | engineering | labs | glossary | references
title = "Vigenère cipher"
subtitle = "Periodic additive substitution over Z26"       # optional, one line
status = "VERIFIED"             # VERIFIED | PROMISING | CLOSED | POWER-LIMITED | INCONCLUSIVE | UNTESTED
status_note = "One or two sentences saying what the status rests on."
family = "Polyalphabetic substitution"   # taxonomy / topic group used for filtering and grouping
tags = ["periodic", "additive"]
related = ["beaufort", "index-of-coincidence"]   # ids of other entries (validated at build time)
summary = "One plain-language paragraph. Searchable."
updated = "2026-09-04"
lab = "vigenere"                # labs only: the Rust lab component key

[[blocks]]                      # ordered page blocks
kind = "definition"             # definition | equations | variants | assumptions | attack | pseudocode
                                # | complexity | failure_modes | controls | example | notes | history
title = "Definition"            # optional; defaults to the kind's display name
body = '''Markdown with $inline$ and $$display$$ LaTeX.'''

[[provenance]]                  # internal evidence (rendered in the Evidence panel)
path = "withmath/wm_core.py"    # repository-relative path
kind = "solver"                 # solver | library | audit | design | note | log | script | data | ledger
note = "Exact period alignment over all shift vectors"
ref = "LOG01"                   # optional ledger/log identifier

[[references]]                  # external literature
title = "The Codebreakers"
author = "David Kahn"
year = 1967
url = "https://…"               # optional
```

Unprefixed provenance paths resolve under `../Cryptanalysis`. `archive-a/` and `archive-b/`
resolve under the additional local research checkouts; `CipherPraxis/` resolves under this
repository. `scripts/check_provenance.py` accepts overrides for all four roots. These aliases
are archival identifiers and do not define the public subject or taxonomy of an article.

Status vocabulary (rendered as badges; the Overview explains them):

- `VERIFIED` — the stated claim has a cited derivation or recorded validation. The status note
  identifies the assumptions, evidence, and scope; the badge does not imply independent peer review.
- `PROMISING` — positive evidence exists but matched controls or audits are incomplete.
- `CLOSED` — a completed search or test produced a documented negative result within a declared
  scope. The entry distinguishes exact exclusion from statistical evidence or a bounded search miss.
- `POWER-LIMITED` — measured recovery or detection was insufficient under the tested conditions
  and budget. A miss gives limited evidence of absence; this is not a proof of impossibility.
- `INCONCLUSIVE` — mixed evidence, unresolved audit, or invalidated run.
- `UNTESTED` — documented but not exercised.

Editorial rules for public fields (`title`, `subtitle`, `summary`, `status_note`, `tags`, `family`,
block `title`/`body`, glossary text):

1. General cryptography only. Forbidden tokens (lint-enforced, case-insensitive, word-bounded):
   `kryptos`, `ctf`, `sanborn`, `langley`, `cia`, `pk1`…`pk10`, `pk89`, `pk98`, `pk8910`, `k1`…`k4`
   as puzzle labels, `leaderboard`, `submission`.
2. Never quote a live ciphertext, plaintext, hint, or crib list. Examples must be generic and
   reproducible (a stated plaintext of your own, a stated key, the resulting ciphertext).
3. Empirical numbers need a receipt (a log, audit, or note named in `[[provenance]]`).
   Mathematical values and synthetic examples need a derivation or mathematical source.
4. Say what a negative means: scope, text length, control power.

## 5. Brand (Dilate, light theme only)

Derived from the Dilate visual language, with restrained scientific reading surfaces:

- Ink `#000020`; secondary ink `rgba(0,0,32,0.68)`; muted ink `#626779`; Dilate blue `#0454ff`;
  slate blue `#3d5b8c`; teal `#35777d`; pale blue tint `#e6edf8`; surfaces `#ffffff`, `#f6f5f4`, `#f0f0eb`,
  `#edede8`; hairline `#e4e4e1`.
- Type: Geist (UI/body, weights 400/500/600), Adamina (serif display/quotes), DM Mono (data/code).
  Letter-spacing −0.01em body, −0.04em display. Radii 8/12/20 px. Generous whitespace.
- Motifs: hairline grids, faint dotted fields, concentric geometry and subtle blue/teal washes.
  Equations and pseudocode use quiet, high-contrast reading panels. Amber marks limited power;
  status text always accompanies color. No dark mode.
- The overview pairs a concise introduction with a transparent vector security illustration in
  blue and teal. The text column stops growing at 640 px; the artwork is centered in the remaining
  space and its faint halo fades into the page. On wide screens the communication chain extends
  left while the main shield stays in place; smaller layouts fit the full illustration to its column.
  The introduction begins 20–32 px below the header. Subject cards use one short description;
  suggested entries and labs link to canonical pages without repeating their metadata and tags.
  The six evidence definitions remain visible, with reading principles open by default in a
  disclosure. Article headers separate parent
  navigation, family and the semantic update date. Responsive overview grids use workspace
  container queries so the left navigation is included in available-width calculations.
- Category indexes use aligned entry rows and a 300 px right sidebar for search, family,
  topic, grouping, and evidence status. All eleven categories share the filter logic;
  Glossary retains its definitions and letter index, and References retains bibliographies.
  Topics are counted once per entry, grouped without regard to case, and alphabetized.
  Full search uses the same row layout and tools rail. Below 920 px of available workspace,
  the rail becomes an expandable panel between the heading and results.
- Labs share sentence-case labels, associated input hints, quiet output panels and a static
  activity marker. The affine lab pairs encryption with recovered plaintext, shows the first
  letter's arithmetic and offers accessible multiplier buttons with their modular inverses.
  Lab container queries keep inputs, output comparisons and the key selector readable at
  the article column's actual width. Shared field grids align labels, controls and helper
  text on separate tracks; result cards share label and value tracks as well. Three-field
  rows stack together on narrow screens, and four-result comparisons become two columns.

## 6. Layout of this directory

```
CipherPraxis/
  ARCHITECTURE.md      this note
  README.md            run/build instructions
  Cargo.toml, Cargo.lock
  content/<section>/*.toml
  crates/core/         content model, search, Markdown+math, crypto core, validators and tests
  crates/web/          Leptos app and labs; build.rs validates and bundles the content
  static/              index.html template assets: styles.css, favicon, fonts CSS
  scripts/             build.sh, serve.py, check.sh
  dist/                build output (generated, ignored)
```
