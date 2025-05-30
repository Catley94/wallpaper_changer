use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize)]
pub struct WHResponse {
    pub data: Vec<WHImageData>,
    pub meta: WHMetaData,
}
#[derive(Debug, Deserialize)]
pub struct WHMetaData {
    pub current_page: i32,
    pub last_page: i32,
    pub per_page: i8,
    pub total: u64,
    pub query: String,
    pub seed: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct WHImageData{
    pub id: String,
    pub url: String,
    pub short_url: String,
    pub views: i64,
    pub favorites: i64,
    pub source: String,
    pub purity: String,
    pub category: String,
    pub dimension_x: i32,
    pub dimension_y: i32,
    pub resolution: String,
    pub ratio: String,
    pub file_size: i64,
    pub file_type: String,
    pub created_at: String,
    pub colors: Vec<String>,
    pub path: String,
    pub thumbs:WHImageThumbnailImageData,
}

#[derive(Debug, Deserialize)]
pub struct WHImageThumbnailImageData {
    pub large: String,
    pub original: String,
    pub small: String,
}