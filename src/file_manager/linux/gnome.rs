use std::process::Command;

pub fn open () {
    // Open Nautilus (typically used in gnome)
    Command::new("nautilus")
        .arg(temp_thumbnail_folder.as_os_str())
        .spawn()
        .expect("Failed to open Nautilus");
}