use std::path::Path;
use std::process::Output;
use crate::utils;
use crate::utils::os::OperatingSystem;

pub mod linux;
pub mod windows;

pub fn change(path: &str) -> std::io::Result<Output> {
    // TODO: Find out what OS is running
    
    if !Path::new(&path).exists() {
        eprintln!("File does not exist {}", &path);
        panic!("File {} does not exist", &path);
    }

    match utils::os::get_operating_system() {
        OperatingSystem::Linux => linux::gnome(path),
        OperatingSystem::Windows => windows::explorer(path),
        OperatingSystem::MacIntel | OperatingSystem::MacArm => std::io::Result::Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "MacOS is not supported yet"
        )),
        OperatingSystem::Unknown => std::io::Result::Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Unknown operating system"
        )),
    }
}