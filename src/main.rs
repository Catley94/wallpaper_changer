use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, copy, Cursor};
use std::path::PathBuf;
use std::path::Path;

mod background;

mod models;
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




    // println!("{:?}", request.body());
    // let response = client.request(request).await.expect("Failed to download image");


    // Get response back from API with query
    let response = ureq::get(&search_query)
        .header("User-Agent", "wallpaper_changer/0.0.1")
        .call()?
        .body_mut()
        .read_json::<wall_haven_models::WHResponse>()?;


    println!("Images per page: {:?}", response.meta.per_page);
    println!("Images on page: {:?}", response.data.len());


    response.data.iter().for_each(|image| {
        // let image = download_image(&image).expect("Failed to download image");
        download_thumbnail(&image, &temp_thumbnail_folder.to_str().unwrap()).expect("Failed to download thumbnail");
    });



    // Get the first image's details
    if let Some(first_image) = response.data.first() {

        // Combine project root with the filename
        let path = project_root
            .join(&first_image.id)
            .to_str()
            .ok_or("Failed to convert path to string").expect("Failed to convert path to string")
            .to_string();

        // background::change(path.as_str());

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

fn download_thumbnail(image: &&WHImageData, local_path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let file_path = format!("{}/{}_thumbnail_small.png", local_path, &image.id);

    // Check if file already exists
    if Path::new(&file_path).exists() {
        println!("File {} already exists, skipping download", file_path);
        return Ok(());
    }
    println!("Image ID: {}", image.id);
    println!("Image URL: {}", image.url);
    println!("Image path: {}", image.path);
    println!("Image thumbs: {}", image.thumbs.small);

    // Create the output file
    let mut image_file = File::create(file_path).expect("Failed to create file");

    let image_request = ureq::get(&image.thumbs.small)
        .call()?;

    let (_, body) = image_request.into_parts();

    let mut bytes_buf: Vec<u8> = Vec::with_capacity(1000);

    body.into_reader()
        .read_to_end(&mut bytes_buf)?;

    // // Copy the response body to the file
    copy(&mut Cursor::new(bytes_buf), &mut image_file)?;
    Ok(())
}