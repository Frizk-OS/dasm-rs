#!/usr/bin/env python3
"""
Port of CtsBuild.mk to Meson build / packaging runner.
Manages cts-testcases output directory and collects CTS distribution artifacts.
"""

import argparse
import os
import shutil
import sys
from pathlib import Path

def main():
    parser = argparse.ArgumentParser(description="CTS Distribution & Testcases Collector (CtsBuild.mk)")
    parser.add_argument("--build-dir", default="build", help="Meson build directory")
    parser.add_argument("--out-dir", default="build/cts-testcases", help="CTS testcases output destination")
    args = parser.parse_args()

    build_dir = Path(args.build_dir)
    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    print(f"=== CTS Testcases Directory: {out_dir} ===")

    # Known artifacts to copy into cts-testcases
    artifacts = [
        # JAR tools & libraries
        build_dir / "tools/cts-xml-generator/cts-xml-generator.jar",
        build_dir / "tools/cts-native-scanner/cts-native-scanner.jar",
        build_dir / "tools/cts-java-scanner/cts-java-scanner.jar",
        build_dir / "tools/cts-api-coverage/cts-api-coverage.jar",
        build_dir / "tools/tradefed-host/cts-tradefed.jar",
        build_dir / "tools/device-setup/ctsdeviceinfolib.jar",
        build_dir / "tools/dex-tools/dex-tools.jar",
        build_dir / "libs/jsonlib.jar",
        build_dir / "libs/ctscommonutil.jar",
        build_dir / "libs/vogarexpectlib.jar",
        build_dir / "libs/ctsdeviceutil.jar",
        build_dir / "libs/ctstestserver.jar",
        build_dir / "libs/wrappedgtest.jar",
        # Host side test JARs
        build_dir / "hostsidetests/CtsUsbTests.jar",
        build_dir / "hostsidetests/CtsMonkeyTestCases.jar",
        build_dir / "hostsidetests/CtsAadbTestCases.jar",
        build_dir / "hostsidetests/CtsSampleHostTestCases.jar",
        build_dir / "hostsidetests/CtsAppSecurityTests.jar",
        build_dir / "hostsidetests/CtsHoloTestCases.jar",
        # Binaries
        build_dir / "tools/dasm-rs/dasm-rs",
        build_dir / "apps/cts-usb-accessory/cts-usb-accessory",
        build_dir / "suite/audio_quality/cts_audio_quality",
        # JNI Libraries
        build_dir / "tests/jni/libcts_jni.so",
        build_dir / "tests/tests/jni/libjnitest.so",
        build_dir / "apps/CtsVerifier/jni/libctsverifier_jni.so",
        build_dir / "suite/cts/deviceTests/dram/jni/libctsdram_jni.so",
        build_dir / "suite/cts/deviceTests/simplecpu/jni/libctscpu_jni.so",
    ]

    copied = 0
    for art in artifacts:
        if art.exists():
            dest = out_dir / art.name
            shutil.copy2(art, dest)
            print(f"  [COLLECT] {art.name} -> {dest}")
            copied += 1

    print(f"Collected {copied} artifacts into {out_dir}")
    return 0

if __name__ == "__main__":
    sys.exit(main())
