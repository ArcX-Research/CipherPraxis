#!/usr/bin/env python3
import argparse
import re
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description="Check that content provenance paths exist")
    parser.add_argument("--content", type=Path, default=Path("content"))
    parser.add_argument("--corpus", type=Path, default=Path("../Cryptanalysis"))
    parser.add_argument("--site", type=Path, default=Path("."))
    args = parser.parse_args()

    missing = []
    records = 0
    files = sorted(args.content.glob("*/*.toml"))
    for source in files:
        text = source.read_text()
        entry = re.search(r'^id = "([^"]+)"$', text, re.MULTILINE)
        entry_id = entry.group(1) if entry else source.stem
        for match in re.finditer(r'^path = "([^"]+)"$', text, re.MULTILINE):
            records += 1
            path = match.group(1)
            if path.startswith("CipherPraxis/"):
                target = args.site / path.removeprefix("CipherPraxis/")
            else:
                target = args.corpus / path
            if not target.exists():
                missing.append((entry_id, path))

    print(
        f"provenance: files={len(files)} records={records} "
        f"missing={len(missing)}"
    )
    for entry_id, path in missing:
        print(f"  [{entry_id}] {path}")
    return int(bool(missing))


if __name__ == "__main__":
    raise SystemExit(main())
