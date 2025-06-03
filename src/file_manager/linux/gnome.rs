use std::path::PathBuf;
use std::process::Command;

pub fn open (path: &PathBuf) {
    // Open Nautilus (typically used in gnome)
    Command::new("nautilus")
        .arg(path.as_os_str())
        .spawn()
        .expect("Failed to open Nautilus");
}