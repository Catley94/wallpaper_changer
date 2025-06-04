use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::path::Path;



mod wallpaper;
mod models;
mod download;
mod utils;
mod file_manager;
mod help_information;

const WALLHAVEN_DIRECT_ID: &str = "https://wallhaven.cc/api/v1/w";
const WALLHAVEN_SEARCH_API: &str = "https://wallhaven.cc/api/v1";
const WALLHAVEN_SEARCH_PARAM: &str = "search?q=";
const WALLHAVEN_SEARCH_PAGE: &str = "page";



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let temp_thumbnail_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp_thumbs");
    let downloaded_images_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("downloaded_images");

    let is_cli = std::env::args().len() > 1;

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

    if args.iter().any(|arg| arg == utils::flags::HELP) {
        // User has passed in --help, display help information
        help_information::display_help_information(args);
        std::process::exit(0);
    }

    let flag_topic = args.iter().any(|arg| arg == utils::flags::TOPIC);
    let flag_change_wallpaper = args.iter().any(|arg| arg == utils::flags::CHANGE_WALLPAPER);
    let flag_page = args.iter().any(|arg| arg == utils::flags::PAGE);

    if flag_page && !flag_change_wallpaper && !flag_topic ||
        flag_page && flag_change_wallpaper && !flag_topic {
        panic!("Error: --page flag must be used with --topic");
    }

    if flag_change_wallpaper {
        let arg_id_value: Option<String> = args.iter()
            .position(|arg: &String| arg == utils::flags::CHANGE_WALLPAPER)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());



        let change_wallpaper_search_query: String = match arg_id_value {
            Some(id) => {
                println!("ID: {:?}", id);
                format!(
                    "{}/{}",
                    WALLHAVEN_DIRECT_ID,
                    id
                )
            },
            None => {
                println!("Error: No ID provided. Please use {} <id>", utils::flags::CHANGE_WALLPAPER);
                return Ok(());
            }
        };


        // Get response back from API with query
        let response = ureq::get(&change_wallpaper_search_query)
            .header("User-Agent", format!("wallpaper_changer/{}", env!("CARGO_PKG_VERSION")))
            .call()?
            .body_mut()
            .read_json::<models::wallhaven::WHDirectModel>()?;

        // println!("{:#?}", response);


        // println!("Image data: {:?}", response.data.path);

        match download::image::original(&response.data, &downloaded_images_folder.to_str().unwrap()) {
            Ok(path) => {
                println!("Downloaded image path: {}", path);
                wallpaper::change(path.as_str()).unwrap();
            },
            Err(e) => println!("Error downloading image: {}", e)
        }
    }

    if flag_topic {

        // TOPIC
        // Download thumbnail images related to topic in temp_thumbs
        // User will then choose background based upon ID
        // Then pass in --change-wallpaper --id <id>

        let current_page: String = "1".to_string();

        // Get the value passed after --topic
        let arg_topic_value: Option<String> = args.iter()
            .position(|arg: &String| arg == utils::flags::TOPIC)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        // Get the value passed after --page
        let arg_page_value: Option<String> = args.iter()
            .position(|arg: &String| arg == utils::flags::PAGE)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        // If --page is passed, use that value, otherwise use the existing current_page value
        let current_page: String = match arg_page_value {
            // Check the value is of Some<T> type and is a valid integer
            Some(page) => {
                if page.parse::<String>().is_ok() {
                    page
                } else {
                    current_page
                }
            },
            None => current_page,
        };
        
        println!("Current page: {}", current_page);


        // Example: https://wallhaven.cc/api/v1/search?q=cats&page=1
        let search_query: String = match arg_topic_value {
            Some(topic) => format!(
                "{}/{}{}&{}={}",
                WALLHAVEN_SEARCH_API,
                WALLHAVEN_SEARCH_PARAM,
                topic,
                WALLHAVEN_SEARCH_PAGE,
                current_page
            ),
            None => {
                println!("Error: No topic provided. Please use {} <search term>", utils::flags::TOPIC);
                return Ok(());  // Exit the function early
            }
        };

        println!("Search query: {}", search_query);

        // Get response back from API with query
        // Returns the page of 24 results
        let response = ureq::get(&search_query)
            .header("User-Agent", format!("wallpaper_changer/{}", env!("CARGO_PKG_VERSION")))
            .call()?
            .body_mut()
            .read_json::<models::wallhaven::WHSearchResponse>()?;


        println!("Images per page: {:?}", response.meta.last_page);

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


        file_manager::linux::gnome::open(&temp_thumbnail_folder);

    }
    Ok(())
}

