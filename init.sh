#!/usr/bin/env bash
# ==============================================================================
# Android CTS Modern - Environment Initialization & Dependency Setup
# ==============================================================================
# Installs and validates all required compilers, runtimes, and build tools:
#   - C/C++ Toolchains: GCC & Clang / LLVM
#   - Rust Toolchain: rustc & cargo (Rust 2024 / stable)
#   - Java Development Kit: OpenJDK 21+ (javac, jar, jarsigner, keytool)
#   - Build Systems: Meson & Ninja
#   - Audio / XML dependencies: alsa-lib / tinyalsa, TinyXML
#   - Android Dex / APK Tooling: D8 / R8 compiler, AAPT
# ==============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

info() {
    echo -e "${BLUE}${BOLD}[INIT]${NC} $1"
}

success() {
    echo -e "${GREEN}${BOLD}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}${BOLD}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}${BOLD}[ERROR]${NC} $1" >&2
}

info "Checking system environment and package manager..."

detect_and_install_packages() {
    if command -v dnf &>/dev/null; then
        info "Detected Fedora / RHEL / CentOS (dnf). Installing packages..."
        sudo dnf install -y \
            gcc gcc-c++ clang llvm lld \
            rust cargo \
            java-latest-openjdk-devel \
            meson ninja-build \
            tinyxml-devel alsa-lib-devel \
            python3 git curl ca-certificates
    elif command -v apt-get &>/dev/null; then
        info "Detected Debian / Ubuntu (apt). Installing packages..."
        sudo apt-get update
        sudo apt-get install -y \
            build-essential gcc g++ clang lld \
            rustc cargo \
            openjdk-21-jdk \
            meson ninja-build \
            libtinyxml-dev libasound2-dev \
            python3 git curl ca-certificates
    elif command -v pacman &>/dev/null; then
        info "Detected Arch Linux (pacman). Installing packages..."
        sudo pacman -S --needed --noconfirm \
            base-devel gcc clang lld \
            rust \
            jdk21-openjdk \
            meson ninja \
            tinyxml alsa-lib \
            python git curl ca-certificates
    elif command -v zypper &>/dev/null; then
        info "Detected openSUSE (zypper). Installing packages..."
        sudo zypper install -y \
            gcc gcc-c++ clang llvm \
            rust cargo \
            java-21-openjdk-devel \
            meson ninja \
            tinyxml-devel alsa-devel \
            python3 git curl ca-certificates
    elif command -v brew &>/dev/null; then
        info "Detected macOS (Homebrew). Installing packages..."
        brew install gcc llvm rust openjdk@21 meson ninja tinyxml python3 git curl
    else
        warn "Could not identify system package manager. Please ensure required compilers are installed manually."
    fi
}

# Ask to install packages if running interactively or if --install flag passed
INSTALL_PKGS=false
for arg in "$@"; do
    case "$arg" in
        --install|-i) INSTALL_PKGS=true ;;
        --help|-h)
            echo "Usage: ./init.sh [options]"
            echo "Options:"
            echo "  --install, -i   Automatically install system dependencies via package manager"
            echo "  --setup-build   Configure Meson build directories (build/ with GCC, build-clang/ with Clang)"
            echo "  --help, -h      Show this help message"
            exit 0
            ;;
    esac
done

if [ "$INSTALL_PKGS" = true ]; then
    detect_and_install_packages
fi

# 1. Verify C/C++ Compilers
info "Verifying C/C++ compilers..."
if command -v gcc &>/dev/null && command -v g++ &>/dev/null; then
    success "GCC found: $(gcc --version | head -n 1)"
else
    warn "GCC / G++ not found in PATH."
fi

if command -v clang &>/dev/null && command -v clang++ &>/dev/null; then
    success "Clang found: $(clang --version | head -n 1)"
else
    warn "Clang / Clang++ not found in PATH."
fi

# 2. Verify Rust & Cargo
info "Verifying Rust toolchain..."
if ! command -v cargo &>/dev/null || ! command -v rustc &>/dev/null; then
    info "Rust not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    source "$HOME/.cargo/env" 2>/dev/null || export PATH="$HOME/.cargo/bin:$PATH"
fi
success "Rust toolchain ready: $(rustc --version) / $(cargo --version)"

# 3. Verify OpenJDK (Target: 21+)
info "Verifying Java JDK..."
if ! command -v javac &>/dev/null; then
    error "javac not found. Please install OpenJDK 21 or newer."
else
    success "Java compiler ready: $(javac --version)"
fi

# 4. Verify Meson & Ninja
info "Verifying Meson & Ninja..."
if ! command -v meson &>/dev/null; then
    error "meson not found. Please install meson (via pip or system package manager)."
fi
if ! command -v ninja &>/dev/null; then
    error "ninja not found. Please install ninja-build."
fi
success "Meson: $(meson --version), Ninja: $(ninja --version)"

# 5. Setup Android D8 / R8 Dex Compiler
info "Checking Android D8 Dex compiler..."
D8_JAR="$SCRIPT_DIR/tools/bin/d8.jar"
mkdir -p "$SCRIPT_DIR/tools/bin"
if [ ! -f "$D8_JAR" ]; then
    info "Downloading D8 / R8 Dex compiler (v8.2.42) from Google Maven repository..."
    curl -sSL -o "$D8_JAR" "https://dl.google.com/dl/android/maven2/com/android/tools/r8/8.2.42/r8-8.2.42.jar"
fi
if [ -f "$D8_JAR" ]; then
    success "D8 Dex compiler ready: $(java -cp "$D8_JAR" com.android.tools.r8.D8 --version)"
else
    warn "D8 Dex compiler could not be initialized."
fi

# 6. Check AAPT & Android SDK Prebuilts
info "Checking Android packaging tools (AAPT & android.jar)..."
AAPT_PATH="$SCRIPT_DIR/../prebuilts/sdk/tools/linux/aapt"
if command -v aapt &>/dev/null; then
    success "System AAPT found: $(aapt version)"
elif [ -x "$AAPT_PATH" ]; then
    success "Tree AAPT found: $("$AAPT_PATH" version)"
else
    warn "AAPT not found in PATH or prebuilts. Building APKs will require AAPT installed."
fi

ANDROID_JAR="$SCRIPT_DIR/../prebuilts/sdk/current/android.jar"
if [ -f "$ANDROID_JAR" ]; then
    success "Android framework prebuilt found: $ANDROID_JAR"
fi

# 7. Configure Meson Build Trees
info "Setting up Meson build directories..."
if [ ! -d "$SCRIPT_DIR/build" ]; then
    info "Setting up GCC build tree (build/)..."
    meson setup build -Denable_rust_jni=true
else
    info "Reconfiguring existing GCC build tree (build/)..."
    meson setup --reconfigure build -Denable_rust_jni=true
fi

if [ ! -d "$SCRIPT_DIR/build-clang" ]; then
    info "Setting up Clang build tree (build-clang/)..."
    CC=clang CXX=clang++ meson setup build-clang -Denable_rust_jni=true
else
    info "Reconfiguring existing Clang build tree (build-clang/)..."
    meson setup --reconfigure build-clang -Denable_rust_jni=true
fi

echo ""
echo -e "${GREEN}${BOLD}==============================================================================${NC}"
echo -e "${GREEN}${BOLD} Android CTS Modern Environment Initialized Successfully!${NC}"
echo -e "${GREEN}${BOLD}==============================================================================${NC}"
echo -e "Quick Commands:"
echo -e "  - Build with GCC:       ${BLUE}ninja -C build${NC}"
echo -e "  - Build with Clang:     ${BLUE}ninja -C build-clang${NC}"
echo -e "  - Run Tests:            ${BLUE}meson test -C build${NC} (or build-clang)"
echo -e "  - Build Distribution:   ${BLUE}ninja -C build cts-distribution${NC}"
echo -e "  - Build All APKs:       ${BLUE}ninja -C build cts-verifier-apk${NC}"
echo -e "${GREEN}${BOLD}==============================================================================${NC}"
