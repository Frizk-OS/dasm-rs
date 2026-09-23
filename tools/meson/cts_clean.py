#!/usr/bin/env python3
"""
Port of CleanSpec.mk to Meson clean-intermediates target.
Removes intermediate artifacts, build caches, and stale objects.
"""

import shutil
import sys
from pathlib import Path

CLEAN_PATHS = [
    "build/vm-tests-tf_intermediates",
    "build/cts-testcases",
    "build/cts-api-coverage",
]

def main():
    root = Path(".")
    print("=== CTS Clean Intermediates (CleanSpec.mk port) ===")
    for path_str in CLEAN_PATHS:
        p = root / path_str
        if p.exists():
            print(f"Removing {p}...")
            if p.is_dir():
                shutil.rmtree(p, ignore_errors=True)
            else:
                p.unlink()
    print("Intermediates cleaned successfully.")
    return 0

if __name__ == "__main__":
    sys.exit(main())
