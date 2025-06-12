use std::error::Error;
use std::path::PathBuf;
use actix_web::{main, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{ Deserialize, Serialize};
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

#[derive(Serialize)]
struct SearchResponse {
    data: models::wallhaven::WHSearchResponse,
    thumbnail_paths: Vec<String>,
}


const WALLHAVEN_DIRECT_ID: &str = "https://wallhaven.cc/api/v1/w";

#[get("/search")]
pub async fn search_theme(params: web::Query<SearchParams>
) -> impl Responder {
    println!("Topic: {}", params.topic);
    println!("Page: {}", params.page);

    // Create thumbnails directory if it doesn't exist
    let thumbnails_folder = utils::get_thumbnails_directory();

    println!("Thumbnails folder: {}", thumbnails_folder.to_str().unwrap());

    if !thumbnails_folder.exists() {
        std::fs::create_dir_all(&thumbnails_folder)
            .expect("Failed to create thumbnails directory");
    }



    let response: Option<models::wallhaven::WHSearchResponse> = utils::create_search_object_response(
        params.topic.clone(),
        params.page
    );

    match response {
        Some(data) => {
            // println!("{:?}", data);

            let mut thumbnail_paths: Vec<String> = Vec::new();

            // Download thumbnails for each image
            for image in &data.data {
                match download::image::thumbnail(&image, &thumbnails_folder.to_str().unwrap()) {
                    Ok(path) => {
                        println!("Successfully downloaded thumbnail for image {}", image.id);
                        println!("Path: {}", path);
                        thumbnail_paths.push(path);
                    },
                    Err(e) => eprintln!("Failed to download thumbnail for image {}: {}", image.id, e)
                }
            }

            // Create combined response
            let combined_response = SearchResponse {
                data,
                thumbnail_paths,
            };



            HttpResponse::Ok().json(combined_response)
        },
        None => HttpResponse::NotFound().finish()
    }
}

#[get("/change-wallpaper")]
async fn change_wallpaper(params: web::Query<ChangeWallpaperParams>) -> Result<HttpResponse, actix_web::Error>
{
    println!("Change wallpaper");

    let downloaded_images_folder = utils::get_downloads_directory();

    println!("Downloaded images folder: {}", downloaded_images_folder.to_str().unwrap());

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
