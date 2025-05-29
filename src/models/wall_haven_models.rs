use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize)]
pub struct WHResponse {
    data: Vec<WHImageData>,
    meta: WHMetaData,
}
#[derive(Debug, Deserialize)]
pub struct WHMetaData {
    current_page: i32,
    last_page: i32,
    per_page: i8,
    total: u64,
    query: String,
    seed: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct WHImageData{
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
pub struct WHImageThumbnailImageData {
    large: String,
    original: String,
    small: String,
}