# Cipher Praxis: A Cryptography Knowledge Base

A public, general-cryptography knowledge base: cipher models, algebra, cryptanalysis techniques,
statistical instruments, search and exact solvers, validation protocols, implementation
engineering and interactive labs, all compiled from Rust to WebAssembly and running entirely in
the browser.

## Run it locally

```bash
cd cipher-praxis
make dev                    # builds, serves, watches and live-reloads
```

Other commands:

```bash
make help                   # list the available development commands
make build                  # optimised release build into ./dist
make build-dev              # fast debug build into ./dist
make serve                  # serve ./dist without rebuilding
make dev PORT=9000          # use a custom development port
make check                  # format, lint, test, content and wasm checks
make content                # content lint only
```

The server starts at `http://127.0.0.1:8787/`. If that port is occupied, it reports the conflict
and uses the next available port.

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

## Evidence requests

Every entry's Evidence panel has a "Request these files" form, because the cited files are internal.
The site is static, so the form's delivery channel is configured in `crates/web/src/state.rs`:

- `EVIDENCE_REQUEST_ENDPOINT = Some("https://…")` posts a JSON document
  (`kind`, `entry`, `title`, `url`, `files`, `name`, `email`, `purpose`, `message`, `submitted_at`)
  and shows the reply status; the endpoint must allow cross-origin `POST` with `Content-Type: application/json`.
- `EVIDENCE_REQUEST_EMAIL = Some("support@questlyst.com")` (the setting in this checkout) opens the visitor's
  mail app through a `mailto:` link with the request pre-filled, and shows the request text plus a direct
  mail link as a fallback.
- With neither set the form copies a plain-text request to the clipboard and shows it, so nothing is
  silently dropped while the channel is unconfigured.
