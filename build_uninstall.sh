#!/bin/bash

PROGRAM_NAME="wallpaper_changer"
ALIASES=("wallpaper_changer")

# Check if running as root
#if [ "$EUID" -ne 0 ]; then
#    echo "Please run as root (with sudo)"
#    exit 1
#fi

# Install the program
uninstall_program() {
    echo "Uninstalling ${PROGRAM_NAME}..."

    echo "Removing dir /usr/share/${PROGRAM_NAME}"
    rm -rf /usr/share/$PROGRAM_NAME

    # Remove symlink(s)
    for alias in "${ALIASES[@]}"; do
      echo "Removing symbolic link in /usr/local/bin for ${alias}"
      rm "/usr/local/bin/${alias}"
    done

    echo "Removing /usr/share/applications/${PROGRAM_NAME}"
    rm /usr/share/applications/${PROGRAM_NAME}
}

# Main
main() {
    uninstall_program

    echo "Uninstallation complete!"
}

main "$@"