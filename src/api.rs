use std::path::PathBuf;
use actix_web::{main, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use crate::{download, models, utils, wallpaper};

#[derive(Deserialize)]
struct SearchParams {
    topic: String,
    page: u16
}

#[derive(Deserialize)]
struct ChangeWallpaperParams {
    id: Option<String>,
}

const WALLHAVEN_DIRECT_ID: &str = "https://wallhaven.cc/api/v1/w";

#[get("/search")]
pub async fn search_theme(params: web::Query<SearchParams>
) -> impl Responder {
    println!("Topic: {}", params.topic);
    println!("Page: {}", params.page);
    let response: Option<models::wallhaven::WHSearchResponse> = utils::create_search_object_response(
        params.topic.clone(),
        params.page
    );

    match response {
        Some(data) => {
            // println!("{:?}", data);
            HttpResponse::Ok().json(data)
        },
        None => HttpResponse::NotFound().finish()
    }
}

#[get("/change-wallpaper")]
async fn change_wallpaper(params: web::Query<ChangeWallpaperParams>) -> Result<HttpResponse, actix_web::Error>
{
    println!("Change wallpaper");

    let downloaded_images_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("downloaded_images");

    let change_wallpaper_search_query: String = match &params.id {
        Some(id) => {
            println!("ID: {:?}", id);
            format!(
                "{}/{}",
                utils::WALLHAVEN_DIRECT_ID,
                id
            )
        },
        None => {
            println!("Error: No ID provided.");
            return Ok(HttpResponse::BadRequest().body("Error: No ID provided in the parameters."));
        }
    };

    // Get response back from API with query
    let response = ureq::get(&change_wallpaper_search_query)
        .header("User-Agent", format!("wallpaper_changer/{}", env!("CARGO_PKG_VERSION")))
        .call()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .body_mut()
        .read_json::<models::wallhaven::WHDirectModel>()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    match download::image::original(&response.data, &downloaded_images_folder.to_str().unwrap()) {
        Ok(path) => {
            println!("Downloaded image path: {}", path);
            wallpaper::change(path.as_str()).unwrap();
        },
        Err(e) => println!("Error downloading image: {}", e)
    }

    Ok(HttpResponse::Ok().finish())
}
