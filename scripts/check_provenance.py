#!/usr/bin/env python3
import argparse
import re
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description="Check that content provenance paths exist")
    parser.add_argument("--content", type=Path, default=Path("content"))
    parser.add_argument("--corpus", type=Path, default=Path("../Cryptanalysis"))
    parser.add_argument("--site", type=Path, default=Path("."))
    research_root = Path.home() / "Projects/Mywork/ArcX-Research/Research/Cryptography"
    parser.add_argument("--archive-a", type=Path, default=research_root / "pk8-research")
    parser.add_argument("--archive-b", type=Path, default=research_root / "pk9pk10-research")
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
            elif path.startswith("archive-a/"):
                target = args.archive_a / path.removeprefix("archive-a/")
            elif path.startswith("archive-b/"):
                target = args.archive_b / path.removeprefix("archive-b/")
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
