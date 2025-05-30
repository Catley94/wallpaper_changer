use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, copy, Cursor};
use std::path::PathBuf;
use std::path::Path;

mod wallpaper;

mod models;
mod download;

use models::wall_haven_models;
use crate::models::wall_haven_models::{WHImageData};

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
        .read_json::<wall_haven_models::WHResponse>()?;


    println!("Images per page: {:?}", response.meta.per_page);


    response.data.iter().for_each(|image_data| {
        match download::image::thumbnail(&image_data, &temp_thumbnail_folder.to_str().unwrap()) {
            Ok(_) => println!(),
            Err(e) => println!("Error downloading image: {}", e)
        }
    });

    // Get the first image's details
    if let Some(first_image) = response.data.first() {

        if let Ok(downloaded_image_path) = download::image::original(&first_image, &project_root.to_str().unwrap()) {
            println!("Downloaded image path: {}", downloaded_image_path);
            
            match wallpaper::change(downloaded_image_path.as_str()) {
                Ok(_) => println!("Wallpaper changed"),
                Err(e) => println!("Error changing wallpaper: {}", e)
            }
        }

        // Combine project root with the filename
        // let path = project_root
        //     .join(&first_image.id)
        //     .to_str()
        //     .ok_or("Failed to convert path to string").expect("Failed to convert path to string")
        //     .to_string();

        // println!("Path: {}", path);
        //
        // wallpaper::change(path.as_str());

    }



    // println!("{:?}", response);

    /*
        TODO
            1. Change wallpaper based upon first image as a test case
            2. Collect all thumbnails from however many images on page (using meta data)
     */

    // 1. Change wallpaper
    


    // 1a. Support Windows / macos and Linux (Debian/Ubuntu, Fedora and Arch based)

    Ok(())
}

// fn download_image(image: &&WHImageData) -> Result<(), Box<dyn Error + Send + Sync>> {
//     // Create the output file
//     let mut image_file = File::create(format!("{}.png", &image.id)).expect("Failed to create file");
//
//     let image_request = ureq::get(&image.path)
//         .call()?;
//
//     let (_, body) = image_request.into_parts();
//
//     let mut bytes_buf: Vec<u8> = Vec::with_capacity(1000);
//
//     body.into_reader()
//         .read_to_end(&mut bytes_buf)?;
//
//     // // Copy the response body to the file
//     copy(&mut Cursor::new(bytes_buf), &mut image_file)?;
//     Ok(())
// }

