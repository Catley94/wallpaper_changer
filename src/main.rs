use std::cell::RefCell;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::path::Path;
use std::rc::Rc;
use gtk4 as gtk;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Entry, Button, Grid, Image, Label, Builder, CssProvider, StyleContext};
use gtk4::gdk::Display;
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
const APP_ID: &str = "org.example.wallpaper_changer";

#[derive(Clone)]
struct AppState {
    current_page: u16,
    current_search: String,
    flow_box: gtk::FlowBox,
    page_label: gtk::Label,
    search_button: gtk::Button,
    prev_button: gtk::Button,
    next_button: gtk::Button,
    loading_label: gtk::Label,
    scroll_window: gtk::ScrolledWindow,
    temp_thumbnail_folder: PathBuf,
    downloaded_images_folder: PathBuf,
}


#[derive(Clone)]
enum WallpaperMessage {
    SetWallpaper(models::wallhaven::WHImageData),
    DownloadImage(models::wallhaven::WHImageData),
    // Add other message types as needed
}


#[derive(Default)]
pub struct WallpaperWindow {
    pub window: ApplicationWindow,
    pub search_entry: Entry,
    pub search_button: Button,
    pub prev_button: Button,
    pub next_button: Button,
    pub page_label: Label,
    pub loading_label: Label,
}

impl WallpaperWindow {
    pub fn new(app: &Application) -> Self {
        // Load the UI file
        let builder = Builder::from_string(include_str!("window.ui"));

        // Get widgets from builder
        let window: ApplicationWindow = builder.object("window").expect("Failed to get window");
        window.set_application(Some(app));

        let search_entry: Entry = builder.object("search_entry").expect("Failed to get search entry");
        let search_button: Button = builder.object("search_button").expect("Failed to get search button");
        let prev_button: Button = builder.object("prev_button").expect("Failed to get prev button");
        let next_button: Button = builder.object("next_button").expect("Failed to get next button");
        let page_label: Label = builder.object("page_label").expect("Failed to get page label");
        let loading_label: Label = builder.object("loading_label")
            .expect("Failed to get loading label");


        // Load CSS
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("style.css"));

        // Add the provider to the default screen
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Add CSS classes to widgets
        search_button.add_css_class("search_button");
        prev_button.add_css_class("prev_button");
        next_button.add_css_class("next_button");
        page_label.add_css_class("page_label");

        Self {
            window,
            search_entry,
            search_button,
            prev_button,
            next_button,
            page_label,
            loading_label
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let temp_thumbnail_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp_thumbs");
    let downloaded_images_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("downloaded_images");

    let is_cli = std::env::args().len() > 1;

    if is_cli {
        cli_mode(temp_thumbnail_folder, downloaded_images_folder)?;
    } else {
        gui_mode();
    }

    Ok(())
}

fn gui_mode() {
    let temp_thumbnail_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp_thumbs");
    let downloaded_images_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("downloaded_images");
    // GUI - gtk4

    // Create a new application
    let app = Application::builder()
        .application_id("com.example.search")
        .build();

    app.connect_activate(move |app| {
        // We create the main window.
        // Create the main window
        let wallpaper_window = WallpaperWindow::new(app);

        // Create a vertical box for layout
        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        // Create search controls
        let search_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();

        let search_entry = Entry::builder()
            .placeholder_text("Enter search topic...")
            .hexpand(true)
            .build();

        let search_button = Button::builder()
            .label("Search")
            .build();

        // Navigation controls box
        let nav_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .halign(gtk::Align::Center)
            .margin_top(10)
            .build();

        let prev_button = Button::builder()
            .label("Previous")
            .sensitive(false)  // Disabled initially
            .build();

        let next_button = Button::builder()
            .label("Next")
            .sensitive(false)  // Disabled initially
            .build();

        let loading_label: Label = gtk::Label::new(Some("Downloading images..."));


        let page_label = gtk::Label::new(Some("Page: 1"));

        nav_box.append(&prev_button);
        nav_box.append(&page_label);
        nav_box.append(&next_button);
        nav_box.append(&loading_label);

        search_box.append(&search_entry);
        search_box.append(&search_button);

        // Create scrollable grid for thumbnails
        let scroll_window = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .build();

        // Instead of creating a Grid, create a FlowBox
        let flow_box = gtk::FlowBox::builder()
            .valign(gtk::Align::Start)
            .max_children_per_line(4)  // Set how many items per row
            .min_children_per_line(2)  // Minimum items per row
            .selection_mode(gtk::SelectionMode::None)
            .homogeneous(true)         // Make all children the same size
            .row_spacing(10)
            .column_spacing(10)
            .margin_top(10)
            .build();

        // Add the FlowBox to the ScrolledWindow instead of Grid
        scroll_window.set_child(Some(&flow_box));

        // Add everything to the main box
        main_box.append(&search_box);
        main_box.append(&nav_box);
        main_box.append(&scroll_window);
        wallpaper_window.window.set_child(Some(&main_box));

        // Set up state
        let current_page = std::rc::Rc::new(std::cell::RefCell::new(1));
        let current_search = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

        // Shared state
        let state = Rc::new(RefCell::new(AppState {
            current_page: 1,
            current_search: String::new(),
            flow_box: flow_box.clone(),
            page_label: page_label.clone(),
            search_button: search_button.clone(),
            prev_button: prev_button.clone(),
            next_button: next_button.clone(),
            loading_label: loading_label.clone(),
            scroll_window: scroll_window.clone(),
            temp_thumbnail_folder: temp_thumbnail_folder.clone(),
            downloaded_images_folder: downloaded_images_folder.clone(),
        }));



        // Search button handler / Handle search button clicks
        let state_search = state.clone();

        set_loading_state(&state.borrow(), false);
        search_button.connect_clicked(move |_| {
            let search_text = search_entry.text().to_string();
            let mut state = state_search.borrow_mut();
            let temp_thumbnail_folder = state.temp_thumbnail_folder.to_str().unwrap();

            download::clear_temp_thumbnails(state.temp_thumbnail_folder.to_str().unwrap());

            if !search_text.is_empty() {
                // let mut state = state_search.borrow_mut();
                state.current_page = 1;
                state.current_search = search_text.clone();

                // Show loading state
                set_loading_state(&state, true);

                // Update page number
                state.page_label.set_text(&format!("Page: {}", state.current_page));
                // Disable previous button
                state.prev_button.set_sensitive(false);
                // Enable next button
                state.next_button.set_sensitive(true);

                // Clear existing thumbnails
                while let Some(child) = state.flow_box.first_child() {
                    state.flow_box.remove(&child);
                }

                // Update the grid with new the new images on the updated page number
                update_grid(
                    &state.flow_box,
                    &search_text,
                    state.current_page,
                    state.temp_thumbnail_folder.clone(),
                    state.downloaded_images_folder.clone()
                );

                // Hide loading label
                set_loading_state(&state, false);
            }
        });

        // Previous button handler
        let state_prev = state.clone();

        prev_button.connect_clicked(move |_| {
            let mut state = state_prev.borrow_mut();
            // let mut page = current_page_clone_pb.borrow_mut();

            // Minus the page number by 1 if greater than 1
            if state.current_page > 1 {
                state.current_page -= 1;
                // Show loading label
                set_loading_state(&state, true);

                // Update page number and set the button to be enabled if the page number is greater than 1
                state.page_label.set_text(&format!("Page: {}", state.current_page));
                state.prev_button.set_sensitive(state.current_page > 1);

                // Clear existing thumbnails
                while let Some(child) = state.flow_box.first_child() {
                    state.flow_box.remove(&child);
                }

                // Update the grid with new the new images on the updated page number
                update_grid(
                    &state.flow_box,
                    &state.current_search,
                    state.current_page,
                    state.temp_thumbnail_folder.clone(),
                    state.downloaded_images_folder.clone()
                );

                // Reset the scroll bar back to the top
                state.scroll_window.vadjustment().set_value(0.0);

                // Hide loading label
                set_loading_state(&state, false);
            }
        });

        // Next button handler
        let state_next = state.clone();

        next_button.connect_clicked(move |button| {
            let mut state = state_next.borrow_mut();
            set_loading_state(&state, true);

            download::clear_temp_thumbnails(state.temp_thumbnail_folder.to_str().unwrap());

            state.current_page += 1;

            // Show loading label

            // Get response data to check last page
            let search_text = state.current_search.to_string();
            let response = create_search_object_response(search_text.clone(), state.current_page);


            // Update page number
            state.page_label.set_text(&format!("Page: {}", state.current_page));

            // Set button sensitivity based on current page and last page from response
            if let Some(response_data) = &response {
                button.set_sensitive(state.current_page < response_data.meta.last_page as u16);
            }

            // Enable the previous button if the page number is greater than 1
            state.prev_button.set_sensitive(state.current_page > 1);

            // Clear existing thumbnails
            while let Some(child) = state.flow_box.first_child() {
                state.flow_box.remove(&child);
            }

            // Update the grid with new the new images on the updated page number
            update_grid(
                &state.flow_box,
                &state.current_search,
                state.current_page,
                state.temp_thumbnail_folder.clone(),
                state.downloaded_images_folder.clone()
            );

            // Reset the scroll bar back to the top
            state.scroll_window.vadjustment().set_value(0.0);

            // Hide loading label
            // TODO: The GUI halts when loading images, so this is never seen
            set_loading_state(&state, false);
        });

        // Show the window.
        wallpaper_window.window.present();
    });

    app.run();
}

fn create_search_object_response(search_text: String, current_page_inner: u16) -> Option<WHSearchResponse> {
    // Example: https://wallhaven.cc/api/v1/search?q=cats&page=1
    let search_query = match create_seach_query_object(Some(search_text), current_page_inner.to_string()) {
        Ok(value) => value,
        Err(_) => String::new()
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
    response
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


        // println!("Images per page: {:?}", response.meta.last_page);

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

// Modify the add_image_to_grid function to work with FlowBox:
fn add_image_to_flow_box(flow_box: &gtk::FlowBox, image_path: &str, image_data: &models::wallhaven::WHImageData, original_downloaded_images_folder: PathBuf
) {
    let image = Image::from_file(image_path);
    image.set_size_request(300, 250);
    image.add_css_class("thumbnail-image");

    // Create a button container for the image
    let button = Button::new();
    button.set_child(Some(&image));

    let image_data = image_data.clone();
    let original_downloaded_folder = original_downloaded_images_folder.clone();

    button.connect_clicked(move |_| {
        // Download and set wallpaper directly
        match download::image::original(&image_data, &original_downloaded_folder.to_str().unwrap()) {
            Ok(path) => {
                println!("Downloaded image path: {}", path);
                match wallpaper::change(path.as_str()) {
                    Ok(_) => {
                        println!("Wallpaper changed successfully");
                    }
                    Err(_) => {
                        println!("Error changing wallpaper");
                    }
                }
            }
            Err(e) => println!("Error downloading image: {}", e)
        }
    });

    flow_box.append(&button);
}

fn update_grid(flow_box: &gtk::FlowBox, search_text: &str, page: u16, temp_thumbnail_folder: PathBuf, downloaded_images_folder: PathBuf) {
    // Get the response with the topic (search_text) and page number (page)
    let response = create_search_object_response(search_text.to_string(), page);

    let thumbnail_paths: Vec<String> = parse_response(&response, &temp_thumbnail_folder);

    if let Some(response_data) = response {
        response_data.data.iter().enumerate().for_each(|(index, image_data)| {
            let local_thumbnail = format!("{}/wallhaven-{}.{}",
                                          temp_thumbnail_folder.to_str().unwrap(),
                                          image_data.id,
                                          utils::get_file_extension(&image_data.file_type)
            );

            add_image_to_flow_box(&flow_box, &local_thumbnail, &image_data, downloaded_images_folder.clone());

        })

    };
}

fn parse_response(response: &Option<models::wallhaven::WHSearchResponse>, temp_thumbnail_folder: &PathBuf) -> Vec<String> {
    let mut thumbnail_paths: Vec<String> = Vec::new();

    if let Some(response_data) = response {
        for image_data in response_data.data.iter() {
            match download::image::thumbnail(&image_data, &temp_thumbnail_folder.to_str().unwrap()) {
                Ok(_) => {
                    // Download succeeded, use image_data's path or URL instead
                    // Construct the correct local thumbnail path
                    let local_thumbnail = format!("{}/wallhaven-{}.{}",
                                                  temp_thumbnail_folder.to_str().unwrap(),
                                                  image_data.id,
                                                  utils::get_file_extension(&image_data.file_type)
                    );

                    // println!("Local Thumbnail: {}", &local_thumbnail);
                    thumbnail_paths.push(local_thumbnail);


                    println!("Downloaded image path: {}", image_data.path);

                },
                Err(e) => println!("Error downloading image: {}", e),
            }
        }
    }
    thumbnail_paths
}

fn set_loading_state(state: &AppState, is_loading: bool) {
    state.loading_label.set_visible(is_loading);
    state.search_button.set_sensitive(!is_loading);
    state.prev_button.set_sensitive(!is_loading && state.current_page > 1);
    state.next_button.set_sensitive(!is_loading);
}
