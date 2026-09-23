#!/usr/bin/env bash
# Helper script to select and configure Meson build with either GCC or Clang
set -e

COMPILER="${1:-gcc}"
BUILD_DIR="${2:-build}"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

case "$COMPILER" in
    gcc)
        echo "==> Configuring Meson build with GCC (native-gcc.ini) into '$BUILD_DIR'..."
        meson setup "$BUILD_DIR" "$ROOT_DIR" \
            --native-file "$ROOT_DIR/tools/meson/native-gcc.ini" \
            --reconfigure || meson setup "$BUILD_DIR" "$ROOT_DIR" \
            --native-file "$ROOT_DIR/tools/meson/native-gcc.ini"
        ;;
    clang)
        echo "==> Configuring Meson build with Clang (native-clang.ini) into '$BUILD_DIR'..."
        meson setup "$BUILD_DIR" "$ROOT_DIR" \
            --native-file "$ROOT_DIR/tools/meson/native-clang.ini" \
            --reconfigure || meson setup "$BUILD_DIR" "$ROOT_DIR" \
            --native-file "$ROOT_DIR/tools/meson/native-clang.ini"
        ;;
    *)
        echo "Usage: $0 [gcc|clang] [build_directory]"
        exit 1
        ;;
esac

echo "==> Build directory '$BUILD_DIR' is ready. To compile, run: ninja -C $BUILD_DIR"
