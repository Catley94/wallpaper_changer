use std::process::Command;
use std::path::Path;

pub fn explorer(path: &str) -> std::io::Result<std::process::Output> {
    if !Path::new(path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Wallpaper file not found"
        ))
    }

    println!("Windows: Setting wallpaper to {}", path);
    // Using PowerShell to set wallpaper on Windows
    Command::new("powershell")
        .args([
            "-command",
            "Set-ItemProperty -path 'HKCU:\\Control Panel\\Desktop\\' -name Wallpaper -value",
            path,
            ";",
            "RUNDLL32.EXE",
            "user32.dll,UpdatePerUserSystemParameters",
            ",1",
            ",True",
        ])
        .output()
}