#!/usr/bin/env python3
"""Dependency-free development server with rebuild, reload, and SPA fallback."""
import argparse
import errno
import http.server
import json
import mimetypes
import os
import subprocess
import sys
import threading
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DIST = ROOT / "dist"
WATCH_DIRS = [ROOT / "crates", ROOT / "content", ROOT / "static"]
WATCH_FILES = [ROOT / "Cargo.toml", ROOT / "Cargo.lock"]
WATCH_EXT = {".rs", ".toml", ".css", ".html", ".svg", ".js", ".md", ".lock"}

mimetypes.add_type("application/wasm", ".wasm")
mimetypes.add_type("text/javascript", ".js")
mimetypes.add_type("image/svg+xml", ".svg")

STATE = {"version": 0, "status": "ok", "output": "", "building": False, "profile": "dev"}
LOCK = threading.Lock()

LIVE_SNIPPET = """
<script>
(() => {
  let seen = null;
  const overlayId = '__praxis_build_overlay';
  const showOverlay = (text) => {
    let el = document.getElementById(overlayId);
    if (!el) {
      el = document.createElement('pre');
      el.id = overlayId;
      el.style.cssText = 'position:fixed;inset:auto 12px 12px 12px;max-height:45vh;overflow:auto;z-index:9999;background:#fff;border:1px solid #f3c9c9;border-left:4px solid #b22222;border-radius:12px;padding:14px 16px;font:12.5px/1.5 "DM Mono",Menlo,monospace;color:#000020;box-shadow:0 20px 50px -20px rgba(0,0,32,.35);white-space:pre-wrap';
      document.body.appendChild(el);
    }
    el.textContent = text;
  };
  const hideOverlay = () => { const el = document.getElementById(overlayId); if (el) el.remove(); };
  const tick = async () => {
    try {
      const r = await fetch('/__dev/status', { cache: 'no-store' });
      const s = await r.json();
      if (seen === null) seen = s.version;
      if (s.status === 'failed') showOverlay('Build failed — fix the error and save again.\\n\\n' + s.output);
      else if (s.building) showOverlay('Rebuilding…');
      else hideOverlay();
      if (s.version !== seen && s.status === 'ok' && !s.building) location.reload();
    } catch {}
    setTimeout(tick, 1000);
  };
  tick();
})();
</script>
"""


def snapshot():
    latest = 0.0
    count = 0
    for d in WATCH_DIRS:
        if not d.exists():
            continue
        for p in d.rglob("*"):
            if p.is_file() and p.suffix in WATCH_EXT and "target" not in p.parts:
                count += 1
                try:
                    latest = max(latest, p.stat().st_mtime)
                except FileNotFoundError:
                    pass
    for f in WATCH_FILES:
        if f.exists():
            count += 1
            latest = max(latest, f.stat().st_mtime)
    return (latest, count)


def build(profile):
    with LOCK:
        STATE["building"] = True
    print(f"▸ build ({profile}) started", flush=True)
    t0 = time.time()
    proc = subprocess.run([str(ROOT / "scripts" / "build.sh"), profile], cwd=ROOT, capture_output=True, text=True)
    out = (proc.stdout or "") + (proc.stderr or "")
    with LOCK:
        STATE["building"] = False
        STATE["output"] = tail(out, 12000)
        if proc.returncode == 0:
            STATE["status"] = "ok"
            STATE["version"] += 1
            print(f"▸ build ok in {time.time() - t0:.1f}s", flush=True)
        else:
            STATE["status"] = "failed"
            print(out, flush=True)
            print(f"▸ build FAILED after {time.time() - t0:.1f}s (see output above)", flush=True)
    return proc.returncode == 0


def tail(s, n):
    return s if len(s) <= n else "…" + s[-n:]


def watcher(profile, interval):
    last = snapshot()
    while True:
        time.sleep(interval)
        cur = snapshot()
        if cur != last:
            time.sleep(interval)
            cur2 = snapshot()
            if cur2 != cur:
                last = cur2
                continue
            last = cur2
            build(profile)


class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(DIST), **kwargs)

    def log_message(self, fmt, *args):
        if self.path.startswith("/__dev/"):
            return
        sys.stdout.write("  %s %s\n" % (self.command, self.path))

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        self.send_header("Cross-Origin-Resource-Policy", "same-origin")
        super().end_headers()

    def do_GET(self):
        if self.path.startswith("/__dev/status"):
            with LOCK:
                body = json.dumps(STATE).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
            return
        path = self.path.split("?", 1)[0].split("#", 1)[0]
        target = DIST / path.lstrip("/")
        if path == "/" or not target.exists() or target.is_dir():
            return self.serve_index()
        return super().do_GET()

    def serve_index(self):
        index = DIST / "index.html"
        if not index.exists():
            body = b"dist/index.html missing: run scripts/build.sh first"
            self.send_response(503)
            self.send_header("Content-Type", "text/plain")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
            return
        html = index.read_text(encoding="utf-8")
        if self.server.live:
            html = html.replace("</body>", LIVE_SNIPPET + "</body>")
        body = html.encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)


class Server(http.server.ThreadingHTTPServer):
    daemon_threads = True
    allow_reuse_address = True
    live = False


def bind_server(host, requested_port, attempts=20):
    last_port = min(requested_port + attempts, 65_536)
    for port in range(requested_port, last_port):
        try:
            return Server((host, port), Handler), port
        except OSError as exc:
            if exc.errno != errno.EADDRINUSE:
                raise
    raise OSError(
        errno.EADDRINUSE,
        f"ports {requested_port}–{last_port - 1} are already in use",
    )


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--port", type=int, default=int(os.environ.get("PORT", "8787")))
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--watch", action="store_true", help="rebuild on source changes and live-reload")
    ap.add_argument("--profile", default="dev", choices=["dev", "release"], help="cargo profile for rebuilds")
    ap.add_argument("--no-build", action="store_true", help="serve the existing dist/ without building first")
    ap.add_argument("--interval", type=float, default=1.0, help="watch poll interval in seconds")
    args = ap.parse_args()
    STATE["profile"] = args.profile
    if not args.no_build:
        if not build(args.profile):
            print("▸ initial build failed; serving anyway so the overlay can show the error", flush=True)
    if args.watch:
        threading.Thread(target=watcher, args=(args.profile, args.interval), daemon=True).start()
    try:
        srv, port = bind_server(args.host, args.port)
    except OSError as exc:
        ap.error(str(exc))
    if port != args.port:
        print(f"▸ port {args.port} is in use; using {port}", flush=True)
    srv.live = args.watch
    url = f"http://{args.host}:{port}/"
    line = f"  Cipher Praxis is running at  {url}  "
    print("", flush=True)
    print("  ┌" + "─" * len(line) + "┐", flush=True)
    print("  │" + line + "│", flush=True)
    print("  └" + "─" * len(line) + "┘", flush=True)
    print(f"  serving {DIST}  ·  watch={'on' if args.watch else 'off'}  ·  profile={args.profile}  ·  Ctrl+C to stop\n", flush=True)
    try:
        srv.serve_forever()
    except KeyboardInterrupt:
        print("\n▸ stopped")
    finally:
        srv.server_close()


if __name__ == "__main__":
    main()
