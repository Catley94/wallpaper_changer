use std::path::Path;
use std::process::Command;
use std::thread;

const RUST_WALLPAPER_CHANGER_NAME: &str = "wallpaper_changer";
const FLUTTER_WALLPAPER_APP_NAME: &str = "wallpaper_app";

fn get_executable_paths() -> (String, String) {
    if cfg!(debug_assertions) {
        // Debug mode - use target/debug paths
        (
            format!("../target/debug/{}", RUST_WALLPAPER_CHANGER_NAME),
            format!("../wallpaper_app/build/linux/x64/debug/bundle/{}", FLUTTER_WALLPAPER_APP_NAME)
        )
    } else {
        // Release mode - use the release paths
        (
            "./apps/wallpaper_changer".to_string(),
            "./apps/bundle/wallpaper_app".to_string()
        )
    }
}


fn main() {
    let (wallpaper_path, flutter_path) = get_executable_paths();

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
