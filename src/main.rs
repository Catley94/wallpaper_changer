use std::env;
use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
struct WHResponse {
    data: Vec<WHImageData>,
    meta: WHMetaData,
}
#[derive(Debug, Deserialize)]
struct WHMetaData {
    current_page: i32,
    last_page: i32,
    per_page: i8,
    total: u64,
    query: String,
    seed: Option<String>,
}
#[derive(Debug, Deserialize)]
struct WHImageData{
    id: String,
    url: String,
    short_url: String,
    views: i64,
    favorites: i64,
    source: String,
    purity: String,
    category: String,
    dimension_x: i32,
    dimension_y: i32,
    resolution: String,
    ratio: String,
    file_size: i64,
    file_type: String,
    created_at: String,
    colors: Vec<String>,
    path: String,
    thumbs:WHImageThumbnailImageData,
}

#[derive(Debug, Deserialize)]
struct WHImageThumbnailImageData {
    large: String,
    original: String,
    small: String,
}
const WALLHAVEN_API: &str = "https://wallhaven.cc/api/v1";
const WALLHAVEN_SEARCH_PARAM: &str = "search?q=";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let topic = &args[1];
    let search_query = format!(
        "{}/{}{}",
        WALLHAVEN_API,
        WALLHAVEN_SEARCH_PARAM,
        topic
    );

    println!("Search query: {}", search_query);

    // let output = Command::new("curl")
    //     .arg("--location")
    //     .arg(&search_query)
    //     .output()?;
    //
    // if output.status.success() {
    //     let response = String::from_utf8(output.stdout)?;
    //     println!("Response: {}", response);
    // } else {
    //     let error = String::from_utf8(output.stderr)?;
    //     eprintln!("Error: {}", error);
    // }

    // Make the request
    // let response: WHResponse = ureq::post("https://wallhaven.cc/api/v1/search?q=naruto")
    //     .header("User-Agent", "wallpaper_changer/0.0.1")
    //     .send_json()

    let body = ureq::get(&search_query)
        .header("User-Agent", "wallpaper_changer/0.0.1")
        .call()?
        .body_mut()
        .read_json::<WHResponse>();

    println!("{:?}", body);


    Ok(())
}