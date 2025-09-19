use std::path::Path;
use std::process::{Output, Child, Command, Stdio};
use std::sync::{Mutex, OnceLock};
use crate::utils;
use crate::utils::os::OperatingSystem;

pub mod linux;
pub mod windows;

// Keep a handle to a running video wallpaper process (if any)
static VIDEO_PROC: OnceLock<Mutex<Option<Child>>> = OnceLock::new();

fn video_proc() -> &'static Mutex<Option<Child>> {
    VIDEO_PROC.get_or_init(|| Mutex::new(None))
}

pub fn stop_video() {
    if let Ok(mut guard) = video_proc().lock() {
        if let Some(mut child) = guard.take() {
            // Try to terminate the process tree politely, then kill
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

pub fn start_video(path: &str) -> std::io::Result<()> {
    if !Path::new(&path).exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Video file not found"));
    }

    // Stop any previous video
    stop_video();

    match utils::os::get_operating_system() {
        OperatingSystem::Linux => {
            // Use xwinwrap + mpv if available to place mpv as desktop background
            // Command example:
            // xwinwrap -ov -fs -- mpv --loop --no-audio --no-input-cursor --no-osd-bar --panscan=1 --wid WID <file>
            let xwinwrap = which::which("xwinwrap").map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "xwinwrap not found. Please install xwinwrap and mpv to enable video wallpapers."))?;
            let mpv = which::which("mpv").map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "mpv not found. Please install mpv to enable video wallpapers."))?;

            let mut cmd = Command::new(xwinwrap);
            cmd.args(["-ov", "-fs", "--"]) 
                .arg(mpv)
                .args(["--loop", "--no-audio", "--no-input-cursor", "--no-osd-bar", "--panscan=1", "--wid", "WID"]) 
                .arg(path)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());

            let child = cmd.spawn()?;
            if let Ok(mut guard) = video_proc().lock() { *guard = Some(child); }
            Ok(())
        }
        OperatingSystem::Windows => {
            Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Video wallpaper not implemented on Windows"))
        }
        OperatingSystem::MacIntel | OperatingSystem::MacArm => {
            Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Video wallpaper not implemented on macOS"))
        }
        OperatingSystem::Unknown => Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Unknown operating system")),
    }
}

pub fn change(path: &str) -> std::io::Result<Output> {
    if !Path::new(&path).exists() {
        eprintln!("File does not exist {}", &path);
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("File {} does not exist", &path)));
    }

    // If a video wallpaper is running, stop it before setting a static image
    stop_video();

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