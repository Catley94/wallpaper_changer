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

    let is_cli = std::env::args().len() > 1;
    let temp_thumbnail_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp_thumbs");
    let downloaded_images_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("downloaded_images");

    if is_cli {
        cli_mode(temp_thumbnail_folder, downloaded_images_folder)?;
    } else {
        // GUI - iced-rs?
    }

    Ok(())
}

fn cli_mode(temp_thumbnail_folder: PathBuf, downloaded_images_folder: PathBuf) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Get the project root path
    // Temp Thumbnail folder for testing purposes

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

        // TOPIC
        // Download thumbnail images related to topic in temp_thumbs
        // User will then choose background based upon ID
        // Then pass in --change-wallpaper --id <id>

        let arg_topic_value: Option<String> = args.iter()
            .position(|arg: &String| arg == "--topic")
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());


        let search_query: String = format!(
            "{}/{}{}",
            WALLHAVEN_SEARCH_API,
            WALLHAVEN_SEARCH_PARAM,
            arg_topic_value.unwrap()
        );

        println!("Search query: {}", search_query);

        // Get response back from API with query
        // Returns the page of 24 results
        let response = ureq::get(&search_query)
            .header("User-Agent", "wallpaper_changer/0.0.1")
            .call()?
            .body_mut()
            .read_json::<models::wallhaven::WHSearchResponse>()?;


        println!("Images per page: {:?}", response.meta.per_page);

        // Collect thumbnail paths
        let mut thumbnail_paths: Vec<String> = Vec::new();

        for image_data in response.data.iter() {
            match download::image::thumbnail(&image_data, &temp_thumbnail_folder.to_str().unwrap().to_string()) {
                Ok(_) => {
                    // Download succeeded, use image_data's path or URL instead
                    println!("Downloaded image path: {}", image_data.path);
                    thumbnail_paths.push(temp_thumbnail_folder.join(Path::new(&image_data.path).file_name().unwrap()).to_str().unwrap().to_string());
                },
                Err(e) => println!("Error downloading image: {}", e),
            }
        }


        // Connect to activate and pass the thumbnail paths
        let paths = thumbnail_paths.clone();
    }
    Ok(())
}

