use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::path::Path;

use iced::widget::{button, column, container, row, text, text_input, Column, Space, Image, Row, image};
use iced::{Application, Command, Element, Length, Sandbox, Settings};
use iced::executor;
use iced::theme::Theme;
use crate::models::wallhaven::WHSearchResponse;

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

struct WallpaperApp {
    search_input: String,
    thumbnails: Vec<String>,
    temp_thumbs_dir: PathBuf,
}

#[derive(Debug, Clone)]
enum Message {
    SearchInputChanged(String),
    SearchPressed,
}

impl Application for WallpaperApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            WallpaperApp {
                search_input: String::new(),
                thumbnails: Vec::new(),
                temp_thumbs_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp_thumbs"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Wallpaper Searcher")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SearchInputChanged(value) => {
                self.search_input = value;
                // println!("Updated Search input: {}", self.search_input);
                Command::none()
            }
            Message::SearchPressed => {
                // Here you would implement the wallhaven search logic
                // Similar to what you have in your CLI mode
                // This should download thumbnails and update self.thumbnails

                let current_page: String = "1".to_string();


                println!("Search input: {}", self.search_input);


                println!("Current page: {}", current_page);


                // Example: https://wallhaven.cc/api/v1/search?q=cats&page=1
                let search_query = match create_seach_query_object(Some(self.search_input.clone()), current_page) {
                    Ok(value) => value,
                    Err(_) => return Command::none(),
                };

                println!("Search query: {}", search_query);

                // Get response back from API with query
                // Returns the page of 24 results
                let response: Option<WHSearchResponse> = match search_topic(&search_query) {
                    Ok(response) => {
                        // println!("Got response: {:?}", response);
                        Some(response)
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        None
                    }
                };

                // println!("Got response: {:?}", response);

                // Collect thumbnail paths
                // let mut thumbnail_paths: Vec<String> = Vec::new();

                if let Some(response_data) = response {
                    for image_data in response_data.data.iter() {
                        match download::image::thumbnail(&image_data, &self.temp_thumbs_dir.to_str().unwrap().to_string()) {
                            Ok(_) => {
                                // Download succeeded, use image_data's path or URL instead
                                // Construct the correct local thumbnail path
                                let local_thumbnail = format!("{}/wallhaven-{}.{}",
                                                              self.temp_thumbs_dir.to_str().unwrap(),
                                                              image_data.id,
                                                              utils::get_file_extension(&image_data.file_type)
                                );

                                // println!("Local Thumbnail: {}", &local_thumbnail);
                                self.thumbnails.push(local_thumbnail);


                                println!("Downloaded image path: {}", image_data.path);

                                // self.thumbnails.push(self.temp_thumbs_dir.join(Path::new(&image_data.path).file_name().unwrap()).to_str().unwrap().to_string());
                            },
                            Err(e) => println!("Error downloading image: {}", e),
                        }
                    }
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let search_row = row![
            text_input("Enter topic...", &self.search_input)
                .on_input(Message::SearchInputChanged)
                .padding(10),
            button("Search")
                .on_press(Message::SearchPressed)
                .padding(10)
        ]
        .spacing(10)
        .padding(20);

        let thumbnails_grid = {
            let mut all_rows = Column::new().spacing(10);
            let mut current_row = Row::new().spacing(10);
            let mut count = 0;

            for (i, thumbnail_path) in self.thumbnails.iter().enumerate() {
                println!("Attempting to load image {}: {:?}", i, thumbnail_path);
                match std::fs::read(thumbnail_path) {
                    Ok(image_bytes) => {
                        println!("Successfully loaded image {}, size: {} bytes", i, image_bytes.len());
                        let handle = image::Handle::from_memory(image_bytes);

                        // let image = Image::<image::Handle>::new("temp_thumbs/wallhaven-1pvwjw.jpg");
                        let image = Image::<image::Handle>::new(thumbnail_path)
                            .width(Length::Fixed(200.0))
                            .height(Length::Fixed(150.0));

                        current_row = current_row.push(
                            // Image::new(handle)
                            // Image::new()
                            image
                        );

                        count += 1;

                        if count % 3 == 0 {
                            all_rows = all_rows.push(current_row);
                            current_row = Row::new().spacing(10);
                        }
                    },
                    Err(e) => {
                        println!("Failed to load image {}: {:?}", i, e);
                        current_row = current_row.push(
                            container(
                                text(format!("Error loading image: {}", e))
                                    .size(14)
                            )
                            .width(Length::Fixed(200.0))
                            .height(Length::Fixed(150.0))
                            .center_x()
                            .center_y()
                        );

                        count += 1;
                        
                        if count % 3 == 0 {
                            all_rows = all_rows.push(current_row);
                            current_row = Row::new().spacing(10);
                        }
                    }
                }
            }

            // Add the last row if it's not complete
            if count % 3 != 0 {
                all_rows = all_rows.push(current_row);
            }

            container(all_rows)
                .width(Length::Fill)
                .center_x()
        };

        container(
            column![
                search_row,
                Space::with_height(Length::Fixed(20.0)),
                thumbnails_grid,
            ]
            .spacing(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into()
    }
}

pub fn run_gui(temp_thumbnail_folder: PathBuf, downloaded_images_folder: PathBuf) -> iced::Result {
    WallpaperApp::run(Settings::default())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let temp_thumbnail_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp_thumbs");
    let downloaded_images_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("downloaded_images");

    let is_cli = std::env::args().len() > 1;

    if is_cli {
        cli_mode(temp_thumbnail_folder, downloaded_images_folder)?;
    } else {
        // GUI - iced-rs?
        run_gui(temp_thumbnail_folder, downloaded_images_folder).expect("GUI Failed to run");
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
        let search_query = match create_seach_query_object(arg_topic_value, current_page) {
            Ok(value) => value,
            Err(value) => return value,
        };

        println!("Search query: {}", search_query);

        // Get response back from API with query
        // Returns the page of 24 results
        let response = search_topic(&search_query)?;


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

fn search_topic(search_query: &String) -> Result<WHSearchResponse, Box<dyn Error + Send + Sync>> {
    let response = ureq::get(search_query.as_str())
        .header("User-Agent", format!("wallpaper_changer/{}", env!("CARGO_PKG_VERSION")))
        .call()?
        .body_mut()
        .read_json::<models::wallhaven::WHSearchResponse>()?;
    Ok(response)
}

fn create_seach_query_object(arg_topic_value: Option<String>, current_page: String) -> Result<String, Result<(), Box<dyn Error + Send + Sync>>> {
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
            return Err(Ok(()));  // Exit the function early
        }
    };
    Ok(search_query)
}