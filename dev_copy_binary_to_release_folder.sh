#!/bin/bash

RUST_PROGRAM_NAME="wallpaper_changer"
FLUTTER_PROGRAM_NAME="wallpaper_app"
APP_RUNNER_PROGRAM_NAME="app_runner"
RELEASE_FOLDER_NAME="release"

# Get version from Cargo.toml
VERSION=$(grep -m1 '^version = ' Cargo.toml | cut -d '"' -f2)

if ! command -v zip >/dev/null 2>&1; then
    echo "Error: zip command not found. Please install zip first"
    exit 1
fi

echo "Building version: linux-release-${VERSION}"

echo "Removing ${RUST_PROGRAM_NAME}.zip"
rm ./${RUST_PROGRAM_NAME}.zip

echo "Removing old release folder"
rm -rf ./$RELEASE_FOLDER_NAME

echo "Building Rust API release version..."
cargo build --release || {
    echo "Build failed!"
    exit 1
}

echo "Building App Runner release version..."
cd app_runner
cargo build --release || {
    echo "Build failed!"
    exit 1
}
cd ..

echo "Building Flutter Linux release..."
cd wallpaper_app && flutter build linux --release || {
    echo "Flutter build failed!"
    exit 1
}
cd ..


echo "Making release folder within project"
mkdir -p ./${RELEASE_FOLDER_NAME}
mkdir -p ./${RELEASE_FOLDER_NAME}/apps

echo "copying (release)Rust API: ${RUST_PROGRAM_NAME} to ./${RELEASE_FOLDER_NAME}"
cp ./target/release/$RUST_PROGRAM_NAME ./${RELEASE_FOLDER_NAME}/apps

echo "copying (release)Flutter App: ${FLUTTER_PROGRAM_NAME} to ./${RELEASE_FOLDER_NAME}"
cp -r ./wallpaper_app/build/linux/x64/release/bundle ./${RELEASE_FOLDER_NAME}/apps

echo "copying (release)Rust - $APP_RUNNER_PROGRAM_NAME to ./${RELEASE_FOLDER_NAME}"
cp ./app_runner/target/release/${APP_RUNNER_PROGRAM_NAME} ./${RELEASE_FOLDER_NAME}

echo "copying release_install.sh to ./${RUST_PROGRAM_NAME}"
cp ./release_install.sh ./${RELEASE_FOLDER_NAME}
echo "copying release_uninstall.sh to ./${RUST_PROGRAM_NAME}"
cp ./release_uninstall.sh ./${RELEASE_FOLDER_NAME}

echo "Creating zip archive..."
zip -r "linux-release-${VERSION}.zip" "./$RELEASE_FOLDER_NAME"

rm -rf ./${RELEASE_FOLDER_NAME}

echo "Done! Created linux-release-v${VERSION}.zip"
