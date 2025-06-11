#!/bin/bash

RUST_PROGRAM_NAME="wallpaper_changer"
FLUTTER_PROGRAM_NAME="wallpaper_app"

#if ! command -v zip >/dev/null 2>&1; then
#    echo "Error: zip command not found. Please install zip first"
#    exit 1
#fi

#echo "Removing ${RUST_PROGRAM_NAME}.zip"
#rm ./${RUST_PROGRAM_NAME}.zip

echo "Building Rust API release version..."
cargo build --release || {
    echo "Build failed!"
    exit 1
}

echo "Building Flutter Linux release..."
cd wallpaper_app && flutter build linux --release || {
    echo "Flutter build failed!"
    exit 1
}
cd ..


echo "Making release folder within project"
mkdir -p ./${RUST_PROGRAM_NAME}

echo "copying (release)Rust API: ${RUST_PROGRAM_NAME} to ./${RUST_PROGRAM_NAME}"
cp ./target/release/$RUST_PROGRAM_NAME ./${RUST_PROGRAM_NAME}

echo "copying (release)Flutter App: ${FLUTTER_PROGRAM_NAME} to ./${RUST_PROGRAM_NAME}"
cp -r ./wallpaper_app/build/linux/x64/release/bundle ./${RUST_PROGRAM_NAME}

#echo "copying release_install.sh to ./${RUST_PROGRAM_NAME}"
#cp release_install.sh ./${RUST_PROGRAM_NAME}
#echo "copying release_uninstall.sh to ./${RUST_PROGRAM_NAME}"
#cp release_uninstall.sh ./${RUST_PROGRAM_NAME}

#echo "Creating zip archive..."
#zip -r "${RUST_PROGRAM_NAME}.zip" "./${RUST_PROGRAM_NAME}"

#rm -rf ./${RUST_PROGRAM_NAME}


echo "Done!"