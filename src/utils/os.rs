#[derive(Debug, PartialEq)]
pub enum OperatingSystem {
    Windows,
    MacIntel,
    MacArm,
    Linux,
    Unknown,
}

pub fn get_operating_system() -> OperatingSystem
{
    if cfg!(target_os = "windows") {
        return OperatingSystem::Windows
    } else if cfg!(target_os = "macos") {
        // Check the architecture
        if cfg!(target_arch = "aarch64") {
            return OperatingSystem::MacArm
        } else if cfg!(target_arch = "x86_64") {
            return OperatingSystem::MacIntel
        } else {
            return OperatingSystem::Unknown
        }
    } else if cfg!(target_os = "linux") {
        return OperatingSystem::Linux
    } else {
        return OperatingSystem::Unknown
    }
}