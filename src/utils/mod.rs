use std::error::Error;
use crate::models;
use std::path::PathBuf;
use crate::utils::os::{ get_operating_system, OperatingSystem};


pub mod flags;
pub mod os;


pub const WALLHAVEN_DIRECT_ID: &str = "https://wallhaven.cc/api/v1/w";
pub const WALLHAVEN_SEARCH_API: &str = "https://wallhaven.cc/api/v1";
pub const WALLHAVEN_SEARCH_PARAM: &str = "search?q=";
pub const WALLHAVEN_SEARCH_PAGE: &str = "page";

pub fn get_file_extension(file_type: &str) -> &str {
    match file_type.to_lowercase().as_str() {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/webp" => "webp",
        _ => "jpg"  // default to jpg if unknown
    }
}

pub fn create_seach_query_object(topic_value: Option<String>, current_page: String) -> Result<String, Result<(), Box<dyn Error + Send + Sync>>> {
    let search_query: String = match topic_value {
        Some(topic) => format!(
            "{}/{}{}&{}={}",
            WALLHAVEN_SEARCH_API,
            WALLHAVEN_SEARCH_PARAM,
            topic,
            WALLHAVEN_SEARCH_PAGE,
            current_page
        ),
        None => {
            eprintln!("Error: No topic provided.");
            return Err(Ok(()));  // Exit the function early
        }
    };
    Ok(search_query)
}

pub fn search_topic(search_query: &String) -> Result<models::wallhaven::WHSearchResponse, Box<dyn Error + Send + Sync>> {
    let response = ureq::get(search_query.as_str())
        .header("User-Agent", format!("wallpaper_changer/{}", env!("CARGO_PKG_VERSION")))
        .call()?
        .body_mut()
        .read_json::<models::wallhaven::WHSearchResponse>()?;
    Ok(response)
}

pub fn create_search_object_response(search_text: String, current_page_inner: u16) -> Option<models::wallhaven::WHSearchResponse> {
    // Example: https://wallhaven.cc/api/v1/search?q=cats&page=1
    let search_query = match create_seach_query_object(Some(search_text), current_page_inner.to_string()) {
        Ok(value) => value,
        Err(_) => String::new()
    };

    println!("Search query: {}", search_query);

    // Get response back from API with query
    // Returns the page of 24 results
    let response: Option<models::wallhaven::WHSearchResponse> = match search_topic(&search_query) {
        Ok(response) => {
            // println!("Got response: {:?}", response);
            Some(response)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    };
    response
}

pub fn get_app_data_directory() -> PathBuf {
    if cfg!(debug_assertions) { // TODO: Currently this is inverted, remove "!"
        // In debug mode, use paths within the project directory
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    } else {
        // In release mode, use paths relative to the executable
        match get_operating_system() {
            OperatingSystem::Windows => {
                // Windows release mode - next to executable
                println!("Windows release mode - next to executable");
                std::env::current_exe()
                    .expect("Failed to get executable path")
                    .parent()
                    .expect("Failed to get executable directory")
                    .to_path_buf()
            },
            OperatingSystem::Linux => {
                // Linux release mode - use /usr/share/wallpaper_changer
                println!("Linux release mode - use /usr/share/wallpaper_changer");
                PathBuf::from("/usr/share/wallpaper_changer") // TODO: MAGIC STRING
            },
            _ => {
                // Fallback to executable directory for unknown/unsupported OS
                println!("Fallback to executable directory for unknown/unsupported OS");
                std::env::current_exe()
                    .expect("Failed to get executable path")
                    .parent()
                    .expect("Failed to get executable directory")
                    .to_path_buf()
            }
        }

    }
}

pub fn get_user_data_directory() -> PathBuf {
    if cfg!(debug_assertions) {
        // In debug mode, use paths within the project directory
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    } else {
        match get_operating_system() {
            OperatingSystem::Windows => {
                // Windows - store user data next to executable
                get_app_data_directory()
            },
            OperatingSystem::Linux => {
                // Linux - use XDG data directory
                println!("Linux - use XDG data directory: {}", std::env::var("USER").unwrap_or_default());
                println!("dirs::data_dir(): {:?}", dirs::data_dir().unwrap().join("wallpaper_changer"));
                
                dirs::data_dir()
                    .unwrap()
                    .join("wallpaper_changer")
                // get_app_data_directory()
            },
            _ => get_app_data_directory()
        }
    }
}


pub fn get_thumbnails_directory() -> PathBuf {
    get_user_data_directory().join("thumbnails")
}

pub fn get_downloads_directory() -> PathBuf {
    get_user_data_directory().join("wallpapers")
}


