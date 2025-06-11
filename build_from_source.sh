#!/bin/bash

RUST_PROGRAM_NAME="wallpaper_changer"
FLUTTER_PROGRAM_NAME="wallpaper_app"
APP_RUNNER_PROGRAM_NAME="app_runner"
ALIASES=("wallpaper_changer")

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root (with sudo)"
    exit 1
fi




# Check if flutter is installed
check_flutter() {
    # At the start of your script, find and store Flutter's path
    FLUTTER_BIN=$(sudo -u "$REAL_USER" bash -l -c 'command -v flutter')
    if [ -z "$FLUTTER_BIN" ]; then
        echo "Error: Flutter not found in PATH. Please make sure Flutter is properly installed."
        exit 1
    fi

    REAL_USER="${SUDO_USER:-$USER}"
    HOME_DIR=$(eval echo ~$REAL_USER)

    # Try to find flutter by running a login shell that sources profile files
    if ! sudo -u "$REAL_USER" bash -l -c 'command -v flutter' >/dev/null 2>&1; then
        echo "Error: Flutter SDK not found. Please install Flutter first"
        echo "Make sure Flutter is in your PATH and properly configured"
        echo "Visit https://docs.flutter.dev/get-started/install/linux"
        exit 1
    fi

}


# Check for Rust toolchain
check_rust() {
    REAL_USER="${SUDO_USER:-$USER}"

    # Check both rustc and cargo using the actual user's environment
    if ! sudo -u "$REAL_USER" bash -c 'source "$HOME/.cargo/env" 2>/dev/null; command -v rustc && command -v cargo' >/dev/null 2>&1; then
        echo "Error: Rust toolchain (rustc/cargo) not found. Please install Rust first:"
        echo "Visit https://rustup.rs/ or run:"
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi

}

# Build release version
build_release() {
    echo "Building Rust API release version..."
    sudo -u "$REAL_USER" bash -c 'source "$HOME/.cargo/env" 2>/dev/null && cargo build --release' || {
        echo "Rust API Build failed!"
        exit 1
    }

    echo "Building Flutter App release version..."
    sudo -u "$REAL_USER" bash -c 'cd ./'"${FLUTTER_PROGRAM_NAME}"' && '"$FLUTTER_BIN"' build linux --release' || {
        echo "Flutter failed!"
        exit 1
    }



    CURRENT_DIR="$PWD"
    echo "Building Rust App Runner release version..."
    sudo -u "$REAL_USER" bash -c "source \"\$HOME/.cargo/env\" 2>/dev/null && cd \"$CURRENT_DIR/$APP_RUNNER_PROGRAM_NAME\" && cargo build --release" || {
        echo "Rust API Build failed!"
        exit 1
    }



}

# Install the program
install_program() {
    echo "Installing ${RUST_PROGRAM_NAME}..."

    echo "Making dir /usr/share/${RUST_PROGRAM_NAME}"
    mkdir -p /usr/share/$RUST_PROGRAM_NAME
    mkdir -p /usr/share/$RUST_PROGRAM_NAME/apps

    # Copy Rust API binary
    cp "./target/release/${RUST_PROGRAM_NAME}" "/usr/share/${RUST_PROGRAM_NAME}/apps"
    chmod +x "/usr/share/${RUST_PROGRAM_NAME}/apps/${RUST_PROGRAM_NAME}"

    # Copy Flutter App binary files
    cp -r "./${FLUTTER_PROGRAM_NAME}/build/linux/x64/release/bundle" "/usr/share/${RUST_PROGRAM_NAME}/apps"
    chmod +x "/usr/share/${RUST_PROGRAM_NAME}/apps/bundle/${FLUTTER_PROGRAM_NAME}"

    # Copy Rust App Runner binary
    cp "./${APP_RUNNER_PROGRAM_NAME}/target/release/${APP_RUNNER_PROGRAM_NAME}" "/usr/share/${RUST_PROGRAM_NAME}/"
    chmod +x "/usr/share/${RUST_PROGRAM_NAME}/${APP_RUNNER_PROGRAM_NAME}"

    # Create symlink(s)
    for alias in "${ALIASES[@]}"; do
      echo "Creating symbolic link in /usr/local/bin for ${alias}"
      ln -sf "/usr/share/${RUST_PROGRAM_NAME}/${APP_RUNNER_PROGRAM_NAME}" "/usr/local/bin/${alias}"
    done

}

# Main
main() {
    check_rust
    check_flutter
    build_release
    install_program

    echo "Installation complete!"
}

main "$@"