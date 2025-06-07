use std::thread::current;
use actix_web::{main, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use crate::create_search_object_response;
use crate::models::wallhaven::WHSearchResponse;

#[derive(Deserialize)]
struct SearchParams {
    topic: String,
    page: u16
}

#[get("/search")]
pub async fn search_theme(params: web::Query<SearchParams>
) -> impl Responder {
    println!("Topic: {}", params.topic);
    println!("Page: {}", params.page);
    let response: Option<WHSearchResponse> = create_search_object_response(
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

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Manual World!")
}

