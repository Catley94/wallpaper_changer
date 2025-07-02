@echo off
setlocal EnableDelayedExpansion

set "RUST_PROGRAM_NAME=wallpaper_changer"
set "FLUTTER_PROGRAM_NAME=wallpaper_app"
set "APP_RUNNER_PROGRAM_NAME=app_runner"
set "RELEASE_FOLDER_NAME=windows-release"

:: Get version from Cargo.toml
for /f "tokens=3" %%i in ('findstr /C:"version = " Cargo.toml') do (
    set VERSION=%%i
    set VERSION=!VERSION:"=!
    goto :found_version
)
:found_version

where powershell >nul 2>&1
if errorlevel 1 (
    echo Error: PowerShell is required for ZIP functionality
    exit /b 1
)

echo Removing %RELEASE_FOLDER_NAME%-*.zip
if exist .\%RELEASE_FOLDER_NAME%-*.zip del .\%RELEASE_FOLDER_NAME%-*.zip

echo Removing old release folder
if exist .\%RELEASE_FOLDER_NAME% rd /S /Q .\%RELEASE_FOLDER_NAME%

echo Building Rust API release version...
cargo build --release
if errorlevel 1 (
    echo Build failed!
    exit /b 1
)

echo Building App Runner release version...
cd app_runner
cargo build --release
if errorlevel 1 (
    echo Build failed!
    exit /b 1
)
cd ..

echo Building Flutter Windows release...
cd wallpaper_app
call flutter build windows --release
cd ..


echo Making release folder within project
mkdir .\%RELEASE_FOLDER_NAME%
mkdir .\%RELEASE_FOLDER_NAME%\apps
mkdir .\%RELEASE_FOLDER_NAME%\thumbnails
mkdir .\%RELEASE_FOLDER_NAME%\downloaded_images


echo copying (release)Rust API: %RUST_PROGRAM_NAME% to .\%RELEASE_FOLDER_NAME%
copy .\target\release\%RUST_PROGRAM_NAME%.exe .\%RELEASE_FOLDER_NAME%\apps

echo copying (release)Flutter App: %FLUTTER_PROGRAM_NAME% to .\%RELEASE_FOLDER_NAME%
xcopy /E /I /Y .\wallpaper_app\build\windows\x64\runner\Release .\%RELEASE_FOLDER_NAME%\apps\bundle

echo copying (release)Rust - %APP_RUNNER_PROGRAM_NAME% to .\%RELEASE_FOLDER_NAME%
copy .\app_runner\target\release\%APP_RUNNER_PROGRAM_NAME%.exe .\%RELEASE_FOLDER_NAME%

echo copying release_install.bat to .\%RELEASE_FOLDER_NAME%
if exist .\release_install.bat copy .\release_install.bat .\%RELEASE_FOLDER_NAME%

echo copying release_uninstall.bat to .\%RELEASE_FOLDER_NAME%
if exist .\release_uninstall.bat copy .\release_uninstall.bat .\%RELEASE_FOLDER_NAME%

echo Creating zip archive...
powershell Compress-Archive -Path .\%RELEASE_FOLDER_NAME% -DestinationPath .\windows-release-!VERSION!.zip -Force




echo Done!
endlocal