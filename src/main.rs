use std::error::Error;
use actix_web::{App, HttpServer};

mod wallpaper;
mod models;
mod download;
mod utils;
mod file_manager;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::search_theme)
            .service(api::change_wallpaper)
            .service(api::create_tag)
            .service(api::tag_image)
            .service(api::list_collections)
            .service(api::change_wallpaper_from_path)
    })
        .bind(("127.0.0.1:8080"))?
        .run()
        .await
}