use std::char::MAX;
use std::fs;
use std::path::Path;

pub mod image;

pub fn clear_temp_thumbnails(temp_folder: &str) -> std::io::Result<()> {
    const MAX_FILES: usize = 1000;
    const MAX_SIZE_BYTES: u64 = 500 * 1024 * 1024; // 500MB in bytes

    let path = Path::new(temp_folder);

    // Check if directory exists
    if !path.exists() {
        return Ok(());
    }

    // Get directory contents
    let entries = fs::read_dir(path)?;

    // Count files and total size
    let mut file_count = 0;
    let mut total_size: u64 = 0;

    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                file_count += 1;
                total_size += metadata.len();
            }
        }
    }

    // Clear directory if either threshold is exceeded
    if file_count > MAX_FILES || total_size > MAX_SIZE_BYTES {
        println!("Cleaning temporary thumbnails folder:");
        println!("Files: {}, Total size: {} MB", file_count, total_size / (1024 * 1024));

        // Read directory again to remove files
        for entry in fs::read_dir(path)? {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Err(e) = fs::remove_file(entry.path()) {
                            eprintln!("Failed to remove file {}: {}",
                                      entry.path().display(), e);
                        }
                    }
                }
            }
        }

        println!("Temporary thumbnails folder cleared");
    }

    Ok(())
}