use std::path::Path;
use std::process::Command;
use std::thread;

const RUST_WALLPAPER_CHANGER_NAME: &str = "wallpaper_changer";
const FLUTTER_WALLPAPER_APP_NAME: &str = "wallpaper_app";

// For development mode vs release mode paths
fn get_app_paths() -> (String, String) {
    let exec_path = std::env::current_exe()
        .expect("Failed to get executable path");
    let exec_dir = exec_path.parent()
        .expect("Failed to get executable directory");

    // Check if we're in the release environment
    if exec_dir.join("apps").exists() {
        // Release mode - use relative paths from executable location
        if cfg!(windows) {
            (
                exec_dir.join("apps")
                    .join("wallpaper_changer.exe")
                    .to_string_lossy()
                    .to_string(),
                exec_dir.join("apps")
                    .join("bundle")
                    .join("wallpaper_app.exe")
                    .to_string_lossy()
                    .to_string()
            )
        } else {
            // Linux paths
            (
                exec_dir.join("apps")
                    .join("wallpaper_changer")
                    .to_string_lossy()
                    .to_string(),
                exec_dir.join("apps")
                    .join("bundle")
                    .join("wallpaper_app")
                    .to_string_lossy()
                    .to_string()
            )
        }
    } else {
        // Development mode - use development paths
        if cfg!(windows) {
            (
                "target\\release\\wallpaper_changer.exe".to_string(),
                "wallpaper_app\\build\\windows\\runner\\Release\\wallpaper_app.exe".to_string()
            )
        } else {
            (
                "target/release/wallpaper_changer".to_string(),
                "wallpaper_app/build/linux/x64/release/bundle/wallpaper_app".to_string()
            )
        }
    }
}




fn main() {
    let (wallpaper_path, flutter_path) = get_app_paths();

    // Start the Flutter app in a separate thread
    thread::spawn(move || {
        println!("Starting Flutter app");
        if Path::new(&flutter_path).exists() {
            match Command::new(&flutter_path).spawn() {
                Ok(_) => println!("Flutter app started successfully"),
                Err(e) => eprintln!("Failed to start Flutter app: {}", e)
            }
        } else {
            eprintln!("Flutter app not found at: {}", &flutter_path);
        }

    });

    // Run the wallpaper_changer
    println!("Starting wallpaper_changer");
    if Path::new(&wallpaper_path).exists() {
        match Command::new(&wallpaper_path).status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("wallpaper_changer exited with error");
                }
            },
            Err(e) => eprintln!("Failed to start wallpaper_changer: {}", e)
        }
    } else {
        eprintln!("wallpaper_changer not found at: {}", wallpaper_path);
    }

}
