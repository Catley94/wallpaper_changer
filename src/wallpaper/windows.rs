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

    // Create and execute the PowerShell script with the more reliable method
    let ps_script = format!(
        "Add-Type @\"
using System;
using System.Runtime.InteropServices;
public class Wallpaper {{
    [DllImport(\"user32.dll\", CharSet = CharSet.Auto)]
    public static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);
}}
\"@;
[Wallpaper]::SystemParametersInfo(0x0014, 0, '{}', 0x01 -bor 0x02)",
        path.replace("'", "''") // Escape single quotes for PowerShell
    );

    let output = Command::new("powershell")
        .args(["-command", &ps_script])
        .output()?;

    // Print the command output
    if output.status.success() {
        println!("Wallpaper set successfully");
    } else {
        if let Ok(stderr) = String::from_utf8(output.clone().stderr) {
            eprintln!("Error setting wallpaper: {}", stderr);
        }
        eprintln!("Command failed with status: {}", output.status);
    }

    Ok(output)
}
