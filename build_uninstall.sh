#!/bin/bash

RUST_PROGRAM_NAME="wallpaper_changer"


# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root (with sudo)"
    exit 1
fi

# Install the program
uninstall_program() {
    echo "Uninstalling ${RUST_PROGRAM_NAME}..."

    echo "Removing dir /usr/share/${RUST_PROGRAM_NAME}"
    rm -rf /usr/share/$RUST_PROGRAM_NAME

}

# Main
main() {
    uninstall_program

    echo "Uninstallation complete!"
}

main "$@"