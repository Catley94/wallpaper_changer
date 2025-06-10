#!/bin/bash

PROGRAM_NAME="wallpaper_changer"
ALIASES=("wallpaper_changer")

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root (with sudo)"
    exit 1
fi

# Check for Rust toolchain
#check_rust() {
#    REAL_USER="${SUDO_USER:-$USER}"
#
#    # Check both rustc and cargo using the actual user's environment
#    if ! sudo -u "$REAL_USER" bash -c 'source "$HOME/.cargo/env" 2>/dev/null; command -v rustc && command -v cargo' >/dev/null 2>&1; then
#        echo "Error: Rust toolchain (rustc/cargo) not found. Please install Rust first:"
#        echo "Visit https://rustup.rs/ or run:"
#        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
#        exit 1
#    fi
#
#}

# Build release version
build_release() {
    echo "Building release version..."
    bash -c "cargo build --release"
#    sudo -u "$REAL_USER" bash -c 'source "$HOME/.cargo/env" 2>/dev/null && cargo build --release' || {
#        echo "Build failed!"
#        exit 1
#    }
}

# Install the program
install_program() {
    echo "Installing ${PROGRAM_NAME}..."

    echo "Making dir /usr/share/${PROGRAM_NAME}"
    mkdir -p /usr/share/$PROGRAM_NAME

    # Copy binary
    cp "./target/release/${PROGRAM_NAME}" "/usr/share/${PROGRAM_NAME}/"
    chmod +x "/usr/share/${PROGRAM_NAME}/${PROGRAM_NAME}"

    # Create symlink(s)
    for alias in "${ALIASES[@]}"; do
      echo "Creating symbolic link in /usr/local/bin for ${alias}"
      ln -sf "/usr/share/${PROGRAM_NAME}/${PROGRAM_NAME}" "/usr/local/bin/${alias}"
    done

    # Create desktop file
#    echo "Running create-desktop-file to create a .desktop file for ${PROGRAM_NAME} to use the GUI, stored in /usr/share/applications"
#    /usr/share/${PROGRAM_NAME}/${PROGRAM_NAME} \
#        --global \
#        --name "${PROGRAM_NAME}" \
#        --exec-path "/usr/share/${PROGRAM_NAME}/${PROGRAM_NAME}" \
#        --terminal-app false \
#        --app-type Application \
#        --categories Development
}

# Main
main() {
    build_release
    install_program

    echo "Installation complete!"
    echo "You can now run '${ALIASES[0]}'}' from anywhere"
}

main "$@"