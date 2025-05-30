use std::env;
use std::error::Error;
use std::io::{ Read };
use std::path::PathBuf;

mod wallpaper;
mod models;
mod download;

use crate::models::wallhaven::{WHImageData};

const WALLHAVEN_API: &str = "https://wallhaven.cc/api/v1";
const WALLHAVEN_SEARCH_PARAM: &str = "search?q=";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let topic = &args[1];
    let search_query = format!(
        "{}/{}{}",
        WALLHAVEN_API,
        WALLHAVEN_SEARCH_PARAM,
        topic
    );
    // Get the project root path
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Temp Thumbnail folder for testing purposes
    let temp_thumbnail_folder = project_root.join("temp_thumbs");

    println!("Search query: {}", search_query);

    // Get response back from API with query
    let response = ureq::get(&search_query)
        .header("User-Agent", "wallpaper_changer/0.0.1")
        .call()?
        .body_mut()
        .read_json::<models::wallhaven::WHResponse>()?;


    println!("Images per page: {:?}", response.meta.per_page);


    response.data.iter().for_each(|image_data| {
        match download::image::thumbnail(&image_data, &temp_thumbnail_folder.to_str().unwrap()) {
            Ok(_) => println!(),
            Err(e) => println!("Error downloading image: {}", e)
        }
    });

    // Get the first image's details - Test purposes
    if let Some(first_image) = response.data.first() {

        if let Ok(downloaded_image_path) = download::image::original(&first_image, &project_root.to_str().unwrap()) {
            println!("Downloaded image path: {}", downloaded_image_path);
            
            match wallpaper::change(downloaded_image_path.as_str()) {
                Ok(_) => println!("Wallpaper changed"),
                Err(e) => println!("Error changing wallpaper: {}", e)
            }
        }

    }


    // TODO: 1a. Support Windows / macos and Linux (Debian/Ubuntu, Fedora and Arch based)

    Ok(())
}


