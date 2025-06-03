use std::path::PathBuf;
use std::process::Command;

pub fn open (path: &PathBuf) {
    // Open Windows Explorer
    Command::new("explorer")
        .arg(path.as_os_str())
        .spawn()
        .expect("Failed to open Windows Explorer");
}