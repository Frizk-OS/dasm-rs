#!/usr/bin/env python3
"""
Android APK Builder for Meson Build System.
Compiles modern Java 17-21 sources, resources, and JNI native libraries into Android APKs.
Uses AAPT, OpenJDK javac/jarsigner, and Android D8/R8 Dex compiler.
"""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


def find_executable(name: str, fallback_path: Path | None = None) -> Path:
    found = shutil.which(name)
    if found:
        return Path(found)
    if fallback_path and fallback_path.is_file() and os.access(fallback_path, os.X_OK):
        return fallback_path
    raise FileNotFoundError(f"Could not locate required executable: {name}")


def ensure_debug_keystore(keystore_path: Path) -> Path:
    if keystore_path.is_file():
        return keystore_path
    keystore_path.parent.mkdir(parents=True, exist_ok=True)
    cmd = [
        "keytool",
        "-genkeypair",
        "-alias", "androiddebugkey",
        "-keypass", "android",
        "-keystore", str(keystore_path),
        "-storepass", "android",
        "-dname", "CN=Android Debug,O=Android,C=US",
        "-validity", "10000",
        "-keyalg", "RSA",
        "-keysize", "2048",
    ]
    subprocess.run(cmd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return keystore_path


def main() -> int:
    parser = argparse.ArgumentParser(description="Build Android APK from sources and resources")
    parser.add_argument("--output", type=Path, required=True, help="Destination .apk file path")
    parser.add_argument("--manifest", type=Path, required=True, help="Path to AndroidManifest.xml")
    parser.add_argument("--res-dir", type=Path, action="append", default=[], help="Resource directory")
    parser.add_argument("--assets-dir", type=Path, default=None, help="Assets directory")
    parser.add_argument("--source-root", type=Path, action="append", default=[], help="Java source directory")
    parser.add_argument("--source-file", type=Path, action="append", default=[], help="Single Java source file")
    parser.add_argument("--dependency-jar", type=Path, action="append", default=[], help="Classpath dependency JAR")
    parser.add_argument("--jni-lib", type=Path, action="append", default=[], help="Shared .so library to include")
    parser.add_argument("--jni-abi", type=str, default="x86_64", help="ABI name for JNI libraries (default: x86_64)")
    parser.add_argument("--android-jar", type=Path, default=None, help="Path to android.jar prebuilt")
    parser.add_argument("--aapt", type=Path, default=None, help="Path to aapt tool")
    parser.add_argument("--d8-jar", type=Path, default=None, help="Path to d8/r8.jar")

    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent.parent

    # 1. Resolve Android SDK prebuilt (android.jar)
    android_jar = args.android_jar
    if not android_jar or not android_jar.is_file():
        default_sdk_jar = repo_root.parent / "prebuilts/sdk/current/android.jar"
        if default_sdk_jar.is_file():
            android_jar = default_sdk_jar
        else:
            raise FileNotFoundError("Could not find android.jar. Pass --android-jar.")

    # 2. Resolve AAPT
    aapt_bin = args.aapt
    if not aapt_bin or not aapt_bin.is_file():
        default_aapt = repo_root.parent / "prebuilts/sdk/tools/linux/aapt"
        aapt_bin = find_executable("aapt", default_aapt)

    # 3. Resolve D8 Dex compiler JAR
    d8_jar = args.d8_jar
    if not d8_jar or not d8_jar.is_file():
        local_d8 = repo_root / "tools/bin/d8.jar"
        if not local_d8.is_file():
            local_d8.parent.mkdir(parents=True, exist_ok=True)
            import urllib.request
            url = "https://dl.google.com/dl/android/maven2/com/android/tools/r8/8.2.42/r8-8.2.42.jar"
            print(f"Downloading D8 Dex compiler to {local_d8}...")
            urllib.request.urlretrieve(url, local_d8)
        d8_jar = local_d8

    # 4. Resolve Java tools
    javac_bin = find_executable("javac")
    jarsigner_bin = find_executable("jarsigner")

    with tempfile.TemporaryDirectory(prefix="cts_apk_build_") as tmp_dir_str:
        tmp_dir = Path(tmp_dir_str)
        gen_dir = tmp_dir / "gen"
        gen_dir.mkdir(parents=True, exist_ok=True)
        classes_dir = tmp_dir / "classes"
        classes_dir.mkdir(parents=True, exist_ok=True)
        dex_dir = tmp_dir / "dex"
        dex_dir.mkdir(parents=True, exist_ok=True)
        temp_apk = tmp_dir / "unsigned.apk"

        # Step A: Generate R.java with AAPT
        aapt_gen_cmd = [
            str(aapt_bin), "package", "-f", "-m",
            "-J", str(gen_dir),
            "-M", str(args.manifest),
            "-I", str(android_jar),
            "--auto-add-overlay",
        ]
        for r in args.res_dir:
            if r.is_dir():
                aapt_gen_cmd.extend(["-S", str(r)])

        subprocess.run(aapt_gen_cmd, check=True)

        # Step B: Gather all Java source files
        java_sources: list[Path] = []
        for root in args.source_root:
            if root.is_dir():
                java_sources.extend(root.rglob("*.java"))
        for sf in args.source_file:
            if sf.is_file():
                java_sources.append(sf)
        java_sources.extend(gen_dir.rglob("*.java"))

        if not java_sources:
            raise RuntimeError("No Java source files found to compile for APK!")

        # Step C: Compile Java files with javac
        classpath_elements = [str(android_jar)]
        for dep in args.dependency_jar:
            if dep.is_file():
                classpath_elements.append(str(dep))
        classpath = os.pathsep.join(classpath_elements)

        sources_file = tmp_dir / "sources.txt"
        with open(sources_file, "w", encoding="utf-8") as f:
            for s in java_sources:
                f.write(f"{s}\n")

        javac_cmd = [
            str(javac_bin),
            "-source", "8",
            "-target", "8",
            "-nowarn",
            "-cp", classpath,
            "-d", str(classes_dir),
            f"@{sources_file}",
        ]
        try:
            subprocess.run(javac_cmd, check=True)
        except subprocess.CalledProcessError:
            # Fallback to --release 21 if needed
            fallback_javac = [
                str(javac_bin),
                "--release", "21",
                "-nowarn",
                "-cp", classpath,
                "-d", str(classes_dir),
                f"@{sources_file}",
            ]
            subprocess.run(fallback_javac, check=True)

        # Step D: Convert compiled classes to classes.dex using D8
        compiled_classes = list(classes_dir.rglob("*.class"))
        if not compiled_classes:
            raise RuntimeError("Compilation produced no .class files!")

        class_list_file = tmp_dir / "classes.txt"
        with open(class_list_file, "w", encoding="utf-8") as f:
            for c in compiled_classes:
                f.write(f"{c}\n")

        d8_cmd = [
            "java",
            "-cp", str(d8_jar),
            "com.android.tools.r8.D8",
            "--output", str(dex_dir),
            "--lib", str(android_jar),
        ]
        for dep in args.dependency_jar:
            if dep.is_file():
                d8_cmd.extend(["--lib", str(dep)])

        # Read class arguments
        d8_cmd.extend([str(c) for c in compiled_classes])
        subprocess.run(d8_cmd, check=True, stdout=subprocess.DEVNULL)

        dex_file = dex_dir / "classes.dex"
        if not dex_file.is_file():
            raise FileNotFoundError(f"D8 did not produce {dex_file}")

        # Step E: Package resources into base APK with AAPT
        aapt_pkg_cmd = [
            str(aapt_bin), "package", "-f",
            "-M", str(args.manifest),
            "-I", str(android_jar),
            "-F", str(temp_apk),
            "--auto-add-overlay",
        ]
        for r in args.res_dir:
            if r.is_dir():
                aapt_pkg_cmd.extend(["-S", str(r)])
        if args.assets_dir and args.assets_dir.is_dir():
            aapt_pkg_cmd.extend(["-A", str(args.assets_dir)])

        subprocess.run(aapt_pkg_cmd, check=True)

        # Step F: Add classes.dex into APK
        shutil.copy2(dex_file, tmp_dir / "classes.dex")
        subprocess.run([str(aapt_bin), "add", "-k", str(temp_apk), "classes.dex"], cwd=tmp_dir, check=True, stdout=subprocess.DEVNULL)

        # Step G: Add JNI shared libraries if present
        if args.jni_lib:
            lib_abi_dir = tmp_dir / "lib" / args.jni_abi
            lib_abi_dir.mkdir(parents=True, exist_ok=True)
            for jni in args.jni_lib:
                if jni.is_file():
                    shutil.copy2(jni, lib_abi_dir / jni.name)
                    rel_path = f"lib/{args.jni_abi}/{jni.name}"
                    subprocess.run([str(aapt_bin), "add", "-k", str(temp_apk), rel_path], cwd=tmp_dir, check=True, stdout=subprocess.DEVNULL)

        # Step H: Sign APK with debug key
        debug_keystore = ensure_debug_keystore(repo_root / "tools/meson/debug.keystore")
        sign_cmd = [
            str(jarsigner_bin),
            "-keystore", str(debug_keystore),
            "-storepass", "android",
            "-keypass", "android",
            str(temp_apk),
            "androiddebugkey",
        ]
        subprocess.run(sign_cmd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

        # Step I: Move to final output
        args.output.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(temp_apk, args.output)
        print(f"APK: {args.output}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
