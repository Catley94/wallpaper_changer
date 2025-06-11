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

# Install the program
install_program() {
    echo "Installing ${RUST_PROGRAM_NAME}..."

    echo "Making dir /usr/share/${RUST_PROGRAM_NAME}"
    mkdir -p /usr/share/$RUST_PROGRAM_NAME
    mkdir -p /usr/share/$RUST_PROGRAM_NAME/apps

    # Create user data directories
    echo "Creating user data directories: ~/.local/share/wallpaper_changer/*"
    mkdir -p ~/.local/share/wallpaper_changer
    mkdir -p ~/.local/share/wallpaper_changer/thumbnails
    mkdir -p ~/.local/share/wallpaper_changer/downloaded_images


    # Copy Rust API binary
    cp "./apps/${RUST_PROGRAM_NAME}" "/usr/share/${RUST_PROGRAM_NAME}/apps"
    chmod +x "/usr/share/${RUST_PROGRAM_NAME}/apps/${RUST_PROGRAM_NAME}"

    # Copy Flutter App binary files
    cp -r "./apps/bundle/" "/usr/share/${RUST_PROGRAM_NAME}/apps"
    chmod +x "/usr/share/${RUST_PROGRAM_NAME}/apps/bundle/${FLUTTER_PROGRAM_NAME}"

    # Copy Rust App Runner binary
    cp "./${APP_RUNNER_PROGRAM_NAME}" "/usr/share/${RUST_PROGRAM_NAME}/"
    chmod +x "/usr/share/${RUST_PROGRAM_NAME}/${APP_RUNNER_PROGRAM_NAME}"

    # Create symlink(s)
    for alias in "${ALIASES[@]}"; do
      echo "Creating symbolic link in /usr/local/bin for ${alias}"
      ln -sf "/usr/share/${RUST_PROGRAM_NAME}/${APP_RUNNER_PROGRAM_NAME}" "/usr/local/bin/${alias}"
    done

}

# Main
main() {
    install_program

    echo "Installation complete!"
}

main "$@"