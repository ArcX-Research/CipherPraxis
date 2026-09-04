# Cipher Praxis — A Dilate Cryptography Knowledge Base

A public, general-cryptography knowledge base: cipher models, algebra, cryptanalysis techniques,
statistical instruments, search and exact solvers, validation protocols, implementation
engineering and interactive labs, all compiled from Rust to WebAssembly and running entirely in
the browser. The sibling `../poemanalysis` repository is the internal evidence corpus that the
content's provenance panels point at; it is never published.

## Run it locally

```bash
cd cipher-praxis
./scripts/dev.sh            # builds (debug profile), serves http://127.0.0.1:8787/, watches and live-reloads
```

Other commands:

```bash
./scripts/build.sh          # optimised release build into ./dist (static files)
./scripts/build.sh dev      # fast debug build into ./dist
python3 scripts/serve.py --no-build            # serve ./dist as is (no watcher)
PORT=9000 ./scripts/dev.sh                      # custom port
./scripts/check.sh          # fmt, tests, content lint, wasm type-check
cargo run -p praxis-core --features authoring --bin praxis-check -- content   # content lint only
```

Requirements (all pinned, nothing downloaded at build time except crates on first build):
Rust ≥ 1.85 with the `wasm32-unknown-unknown` target, the `wasm-bindgen` CLI at the version pinned
in `Cargo.toml`/`Cargo.lock` (0.2.127), Python 3 for the dev server. `wasm-opt` is used when
present but not required.

## Layout

```
cipher-praxis/
  ARCHITECTURE.md      architecture, content schema, brand tokens, status vocabulary
  content/             one TOML file per entry, content/<section>/<id>.toml (see content/AUTHORING.md)
  crates/core          content model + validator/lint, search index, Markdown+MathML, cipher/statistics engine (native tests)
  crates/web           Leptos client-side app; build.rs validates and bundles content into the binary
  static/              index.html, styles.css, favicon.svg
  scripts/             dev.sh, build.sh, serve.py, check.sh
  dist/                build output (generated)
```

## Adding content

1. Copy an existing entry in `content/<section>/`, keep the file name equal to the `id`.
2. Follow `content/AUTHORING.md`: general cryptography only, generic examples, every number
   backed by a `[[provenance]]` receipt, status from the fixed vocabulary.
3. Run `./scripts/check.sh` (or just `./scripts/dev.sh`, which rebuilds and shows lint errors in
   the terminal and in the page overlay).

The build fails on forbidden tokens, duplicate ids, missing status notes, unknown lab keys or
bad provenance; dangling `related` links are warnings.
