#!/usr/bin/env python3
"""
Port of CtsCoverage.mk to Meson run target.
Generates CTS API coverage reports (HTML / XML) using cts-api-coverage.jar.
"""

import argparse
import os
import subprocess
import sys
from pathlib import Path

def main():
    parser = argparse.ArgumentParser(description="Generate CTS API Coverage Report")
    parser.add_argument("--mode", choices=["test", "verifier", "combined"], required=True,
                        help="Coverage report mode")
    parser.add_argument("--api-xml", default="build/cts-api-coverage/api.xml",
                        help="Path to API XML definition")
    parser.add_argument("--jar", default="build/tools/cts-api-coverage/cts-api-coverage.jar",
                        help="Path to cts-api-coverage.jar")
    parser.add_argument("--format", default="html", choices=["html", "xml"],
                        help="Output format")
    parser.add_argument("--output", default=None,
                        help="Output report destination path")
    parser.add_argument("apks", nargs="*", help="APK files to scan")

    args = parser.parse_args()

    if not args.output:
        args.output = f"build/cts-api-coverage/{args.mode}-coverage.{args.format}"

    out_file = Path(args.output).resolve()
    out_file.parent.mkdir(parents=True, exist_ok=True)

    report_title = {
        "test": "CTS Tests API Coverage Report",
        "verifier": "CTS Verifier API Coverage Report",
        "combined": "CTS Combined API Coverage Report",
    }[args.mode]

    print(f"=== {report_title} ===")
    print(f"Target output: {out_file}")

    if not os.path.exists(args.jar):
        print(f"Warning: {args.jar} not found. Building it first via meson/ninja...")
        res = subprocess.run(["ninja", "-C", "build", "tools/cts-api-coverage/cts-api-coverage.jar"])
        if res.returncode != 0:
            print("Failed to build cts-api-coverage.jar", file=sys.stderr)
            return res.returncode

    if not args.apks:
        print(f"No APKs specified for {args.mode} mode. Creating placeholder report.")
        with open(out_file, "w", encoding="utf-8") as f:
            f.write(f"<!DOCTYPE html><html><head><title>{report_title}</title></head>")
            f.write(f"<body><h1>{report_title}</h1><p>Status: Ready. No APK packages scanned.</p></body></html>\n")
        print(f"Report written to file://{out_file}")
        return 0

    cmd = [
        "java", "-jar", args.jar,
        "-f", args.format,
        "-o", str(out_file),
    ]
    if os.path.exists(args.api_xml):
        cmd.extend(["-a", args.api_xml])

    cmd.extend(args.apks)

    print(f"Running: {' '.join(cmd)}")
    res = subprocess.run(cmd)
    if res.returncode == 0:
        print(f"{report_title}: file://{out_file}")
    return res.returncode

if __name__ == "__main__":
    sys.exit(main())
