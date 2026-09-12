#!/usr/bin/env python3
"""Give the WASM, its matching JS loader, and CSS independent content-addressed URLs."""
import hashlib
from pathlib import Path
import sys


def fingerprint_assets(dist: Path) -> None:
    pkg = dist / "pkg"
    wasm = pkg / "praxis_bg.wasm"
    js = pkg / "praxis.js"
    css = dist / "styles.css"
    index = dist / "index.html"

    def fingerprint(data: bytes) -> str:
        return hashlib.sha256(data).hexdigest()[:20]

    wasm_name = f"praxis.{fingerprint(wasm.read_bytes())}_bg.wasm"
    loader = js.read_text(encoding="utf-8")
    if "'praxis_bg.wasm'" not in loader:
        raise ValueError("wasm-bindgen loader no longer has the expected WASM URL")
    loader = loader.replace("'praxis_bg.wasm'", repr(wasm_name))
    loader_bytes = loader.encode("utf-8")
    js_name = f"praxis.{fingerprint(loader_bytes)}.js"
    css_name = f"styles.{fingerprint(css.read_bytes())}.css"

    html = index.read_text(encoding="utf-8")
    for old, new in (
        ("/pkg/praxis.js", f"/pkg/{js_name}"),
        ("/pkg/praxis_bg.wasm", f"/pkg/{wasm_name}"),
        ("/styles.css", f"/pkg/{css_name}"),
    ):
        if old not in html:
            raise ValueError(f"HTML shell is missing its expected asset URL: {old}")
        html = html.replace(old, new)

    wasm.rename(pkg / wasm_name)
    (pkg / js_name).write_bytes(loader_bytes)
    js.unlink()
    css.rename(pkg / css_name)
    index.write_text(html, encoding="utf-8")


if __name__ == "__main__":
    fingerprint_assets(Path(sys.argv[1]))
