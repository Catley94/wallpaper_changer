use std::env;

mod models;
use models::wall_haven_models;


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

    let body = ureq::get(&search_query)
        .header("User-Agent", "wallpaper_changer/0.0.1")
        .call()?
        .body_mut()
        .read_json::<wall_haven_models::WHResponse>();

    println!("{:?}", body);


    Ok(())
}