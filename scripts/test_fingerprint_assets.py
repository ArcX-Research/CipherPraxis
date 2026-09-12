"""Regression checks for release mixing and stale styles in the browser cache."""
from pathlib import Path
import re
import tempfile
import unittest

from fingerprint_assets import fingerprint_assets


class FingerprintTests(unittest.TestCase):
    def package(self, wasm=b"wasm release one", js_suffix="", css="body { color: blue }"):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        dist = Path(temporary.name)
        (dist / "pkg").mkdir()
        (dist / "pkg/praxis_bg.wasm").write_bytes(wasm)
        (dist / "pkg/praxis.js").write_text(
            "const wasm = new URL('praxis_bg.wasm', import.meta.url);" + js_suffix
        )
        (dist / "styles.css").write_text(css)
        shell = Path(__file__).resolve().parent.parent / "static/index.html"
        (dist / "index.html").write_bytes(shell.read_bytes())
        fingerprint_assets(dist)
        return {p.relative_to(dist).as_posix(): p.read_bytes() for p in dist.rglob("*") if p.is_file()}

    def asset(self, files, suffix):
        return next(name for name in files if name.endswith(suffix))

    def test_shell_and_loader_point_to_the_same_existing_wasm(self):
        files = self.package()
        wasm = self.asset(files, ".wasm")
        js = self.asset(files, ".js")
        css = self.asset(files, ".css")
        html = files["index.html"].decode()
        self.assertIn(f"'{Path(wasm).name}'", files[js].decode())
        for name in (wasm, js, css):
            self.assertIn("/" + name, html)
        # Every local import/preload/style/init URL must exist in this release.
        for url in re.findall(r"/pkg/[\w.]+", html):
            self.assertIn(url.lstrip("/"), files)
        self.assertNotIn("?v=", html)
        self.assertNotIn("pkg/praxis_bg.wasm", files)
        self.assertNotIn("pkg/praxis.js", files)
        self.assertNotIn("styles.css", files)

    def test_new_wasm_gets_a_new_loader_url_even_when_bindings_are_unchanged(self):
        before = self.package()
        after = self.package(wasm=b"wasm release two")
        self.assertNotEqual(self.asset(before, ".wasm"), self.asset(after, ".wasm"))
        self.assertNotEqual(self.asset(before, ".js"), self.asset(after, ".js"))
        self.assertEqual(self.asset(before, ".css"), self.asset(after, ".css"))

    def test_js_changes_do_not_reuse_immutable_urls(self):
        before = self.package()
        after = self.package(js_suffix="\n// updated loader")
        self.assertNotEqual(self.asset(before, ".js"), self.asset(after, ".js"))
        self.assertEqual(self.asset(before, ".wasm"), self.asset(after, ".wasm"))

    def test_css_only_edits_invalidate_styles_without_redownloading_wasm(self):
        before = self.package()
        after = self.package(css="body { color: teal }")
        self.assertNotEqual(self.asset(before, ".css"), self.asset(after, ".css"))
        self.assertEqual(self.asset(before, ".js"), self.asset(after, ".js"))
        self.assertEqual(self.asset(before, ".wasm"), self.asset(after, ".wasm"))

    def test_identical_inputs_are_reproducible_in_different_directories(self):
        self.assertEqual(self.package(), self.package())


if __name__ == "__main__":
    unittest.main()
