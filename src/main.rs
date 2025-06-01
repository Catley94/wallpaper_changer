use std::env;
use std::error::Error;
use std::io::{ Read };
use std::path::PathBuf;
use std::path::Path;



mod wallpaper;
mod models;
mod download;

use crate::models::wallhaven::{WHImageData};

const WALLHAVEN_DIRECT_ID: &str = "https://wallhaven.cc/api/v1/w";
const WALLHAVEN_SEARCH_API: &str = "https://wallhaven.cc/api/v1";
const WALLHAVEN_SEARCH_PARAM: &str = "search?q=";



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // Get the project root path
    // Temp Thumbnail folder for testing purposes
    let temp_thumbnail_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp_thumbs");
    let downloaded_images_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("downloaded_images");


    let args: Vec<String> = env::args().collect();

    let flag_topic = args.iter().any(|arg| arg == "--topic");
    let flag_change_wallpaper = args.iter().any(|arg| arg == "--change-wallpaper");
    let flag_id = args.iter().any(|arg| arg == "--id");

    if flag_change_wallpaper && flag_id {
        
        let arg_id_value: Option<String> = args.iter()
            .position(|arg: &String| arg == "--id")
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        println!("ID: {:?}", arg_id_value);

        let change_wallpaper_search_query = format!(
            "{}/{}",
            WALLHAVEN_DIRECT_ID,
            arg_id_value.unwrap()
        );


        // Get response back from API with query
        let response = ureq::get(&change_wallpaper_search_query)
            .header("User-Agent", format!("wallpaper_changer/{}", env!("CARGO_PKG_VERSION")))
            .call()?
            .body_mut()
            .read_json::<models::wallhaven::WHDirectModel>()?;

        println!("{:#?}", response);


        println!("Image data: {:?}", response.data.path);

        match download::image::original(&response.data, downloaded_images_folder.to_str().unwrap()) {
            Ok(path) => {
                println!("Downloaded image path: {}", path);
                wallpaper::change(path.as_str()).unwrap();
            },
            Err(e) => println!("Error downloading image: {}", e)
        }


    } else {
        let topic = &args[1];
        let search_query = format!(
            "{}/{}{}",
            WALLHAVEN_SEARCH_API,
            WALLHAVEN_SEARCH_PARAM,
            topic
        );

        println!("Search query: {}", search_query);

        // Get response back from API with query
        let response = ureq::get(&search_query)
            .header("User-Agent", "wallpaper_changer/0.0.1")
            .call()?
            .body_mut()
            .read_json::<models::wallhaven::WHSearchResponse>()?;


        println!("Images per page: {:?}", response.meta.per_page);

        // Collect thumbnail paths
        let mut thumbnail_paths: Vec<String> = Vec::new();

        for image_data in response.data.iter() {
            if let Ok(path) = download::image::thumbnail(&image_data, &temp_thumbnail_folder.to_str().unwrap().to_string()) {
                // Download succeeded, use image_data's path or URL instead
                thumbnail_paths.push(temp_thumbnail_folder.join(Path::new(&image_data.path).file_name().unwrap()).to_str().unwrap().to_string());
            }
        }




        // Connect to activate and pass the thumbnail paths
        let paths = thumbnail_paths.clone();



        // Get the first image's details - Test purposes
        if let Some(first_image) = response.data.first() {

            if let Ok(downloaded_image_path) = download::image::original(&first_image, &downloaded_images_folder.to_str().unwrap()) {
                println!("Downloaded image path: {}", downloaded_image_path);

                match wallpaper::change(downloaded_image_path.as_str()) {
                    Ok(_) => println!("Wallpaper changed"),
                    Err(e) => println!("Error changing wallpaper: {}", e)
                }
            }

        }
    }


    

    Ok(())
}

