pub fn gnome(path: &str) -> std::io::Result<std::process::Output> {
    // Use gsettings to set the wallpaper
    std::process::Command::new("gsettings")
        .args(&[
            "set",
            "org.gnome.desktop.background",
            "picture-uri",
            &format!("file://{}", path),
        ])
        .output()
}