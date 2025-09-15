use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
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

    // Replace spaces with %20 in the topic
    let encoded_topic = params.topic.replace(' ', "%20");

    // Create thumbnails directory if it doesn't exist
    let thumbnails_folder = utils::get_thumbnails_directory();

    println!("Thumbnails folder: {}", thumbnails_folder.to_str().unwrap());

    if !thumbnails_folder.exists() {
        std::fs::create_dir_all(&thumbnails_folder)
            .expect("Failed to create thumbnails directory");
    }

    let response: Option<models::wallhaven::WHSearchResponse> = utils::create_search_object_response(
        encoded_topic.clone(),
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
pub async fn change_wallpaper(params: web::Query<ChangeWallpaperParams>) -> Result<HttpResponse, actix_web::Error>
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

#[derive(Deserialize)]
pub struct CreateTagBody { pub name: String }

#[post("/collections/tags")]
pub async fn create_tag(body: web::Json<CreateTagBody>) -> impl Responder {
    let tag = utils::sanitize_tag_name(&body.name);
    if tag.is_empty() { return HttpResponse::BadRequest().body("Invalid tag name"); }
    let collections_dir = utils::get_collections_directory();
    if let Err(e) = utils::ensure_dir(&collections_dir) { return HttpResponse::InternalServerError().body(format!("Failed to init collections dir: {}", e)); }
    let tag_dir = collections_dir.join(&tag);
    match utils::ensure_dir(&tag_dir) {
        Ok(_) => {
            #[derive(Serialize)]
            struct CreateTagResp { tag: String, path: String }
            let resp = CreateTagResp { tag, path: tag_dir.to_string_lossy().to_string() };
            HttpResponse::Ok().json(resp)
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create tag: {}", e))
    }
}

#[derive(Deserialize)]
pub struct TagImageBody { pub id: String, pub tag: String }

#[post("/collections/tag-image")]
pub async fn tag_image(body: web::Json<TagImageBody>) -> impl Responder {
    let tag = utils::sanitize_tag_name(&body.tag);
    if tag.is_empty() { return HttpResponse::BadRequest().body("Invalid tag name"); }

    // Ensure tag folder exists
    let collections_dir = utils::get_collections_directory();
    if let Err(e) = utils::ensure_dir(&collections_dir) { return HttpResponse::InternalServerError().body(format!("Failed to init collections dir: {}", e)); }
    let tag_dir = collections_dir.join(&tag);
    if let Err(e) = utils::ensure_dir(&tag_dir) { return HttpResponse::InternalServerError().body(format!("Failed to create tag dir: {}", e)); }

    // Fetch image metadata by ID
    let change_wallpaper_search_query = format!("{}/{}", utils::WALLHAVEN_DIRECT_ID, body.id);
    let direct = match ureq::get(&change_wallpaper_search_query)
        .header("User-Agent", format!("wallpaper_changer/{}", env!("CARGO_PKG_VERSION")))
        .call() {
        Ok(mut resp) => {
            match resp.body_mut().read_json::<models::wallhaven::WHDirectModel>() {
                Ok(json) => json,
                Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to parse image info: {}", e))
            }
        },
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to fetch image info: {}", e))
    };

    // Ensure image downloaded to downloads directory
    let downloads = utils::get_downloads_directory();
    if let Err(e) = utils::ensure_dir(&downloads) { return HttpResponse::InternalServerError().body(format!("Failed to init downloads dir: {}", e)); }
    let image_path = match download::image::original(&direct.data, downloads.to_str().unwrap()) {
        Ok(p) => p,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to download image: {}", e))
    };

    // Copy into tag folder
    let src = PathBuf::from(&image_path);
    let file_name = src.file_name().unwrap_or_default();
    let dest = tag_dir.join(file_name);
    if !dest.exists() {
        if let Err(e) = fs::copy(&src, &dest) { return HttpResponse::InternalServerError().body(format!("Failed to copy image: {}", e)); }
    }

    #[derive(Serialize)]
    struct TagImageResp { tag: String, copied_to: String }
    let resp = TagImageResp { tag, copied_to: dest.to_string_lossy().to_string() };
    HttpResponse::Ok().json(resp)
}

#[derive(Serialize)]
pub struct CollectionItem { pub name: String, pub images: Vec<String> }

#[derive(Serialize)]
pub struct CollectionsResp { pub tags: Vec<CollectionItem> }

#[get("/collections")]
pub async fn list_collections() -> impl Responder {
    let collections_dir = utils::get_collections_directory();
    if let Err(e) = utils::ensure_dir(&collections_dir) { return HttpResponse::InternalServerError().body(format!("Failed to init collections dir: {}", e)); }
    let mut items: Vec<CollectionItem> = Vec::new();
    let read_dir = match fs::read_dir(&collections_dir) { Ok(rd) => rd, Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to read collections: {}", e)) };
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let mut images: Vec<String> = Vec::new();
            if let Ok(imgs) = fs::read_dir(&path) {
                for img in imgs.flatten() {
                    let p = img.path();
                    if p.is_file() {
                        let s = p.to_string_lossy().to_string();
                        images.push(s);
                    }
                }
            }
            items.push(CollectionItem { name, images });
        }
    }
    let resp = CollectionsResp { tags: items };
    HttpResponse::Ok().json(resp)
}
