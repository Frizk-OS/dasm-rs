#!/usr/bin/env python3
import sys
import subprocess
import shutil
from pathlib import Path

def main():
    if len(sys.argv) < 5:
        print("Usage: build_cargo_bin.py <cargo_exe> <manifest_path> <target_dir> <output_bin>", file=sys.stderr)
        return 1

    cargo = sys.argv[1]
    manifest = Path(sys.argv[2]).resolve()
    target_dir = Path(sys.argv[3]).resolve()
    output_bin = Path(sys.argv[4]).resolve()

    cmd = [
        cargo, "build", "--release",
        "--manifest-path", str(manifest),
        "--target-dir", str(target_dir),
    ]

    res = subprocess.run(cmd)
    if res.returncode != 0:
        return res.returncode

    built_bin = target_dir / "release" / output_bin.name
    if not built_bin.exists():
        print(f"Error: expected built binary at {built_bin} not found", file=sys.stderr)
        return 2

    output_bin.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(built_bin, output_bin)
    output_bin.chmod(0o755)
    return 0

if __name__ == "__main__":
    sys.exit(main())
