use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct WHSearchResponse {
    pub data: Vec<WHImageData>,
    pub meta: WHSearchMetaData,
}
#[derive(Debug, Deserialize, Clone)]
pub struct WHSearchMetaData {
    pub current_page: i32,
    pub last_page: i32,
    pub per_page: i8,
    pub total: u64,
    pub query: String,
    pub seed: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WHDirectModel {
    pub data: WHImageData,
}


#[derive(Debug, Deserialize, Clone)]
pub struct WHTag {
    pub id: i32,
    pub name: String,
    pub alias: String,
    pub category_id: i32,
    pub category: String,
    pub purity: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WHAvatar {
    #[serde(rename = "200px")]
    pub px200: String,
    #[serde(rename = "128px")]
    pub px128: String,
    #[serde(rename = "32px")]
    pub px32: String,
    #[serde(rename = "20px")]
    pub px20: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WHUploader {
    pub username: String,
    pub group: String,
    pub avatar: WHAvatar,
}


#[derive(Debug, Deserialize, Clone)]
pub struct WHImageData {
    pub id: String,
    pub url: String,
    pub short_url: String,
    #[serde(default)]  // This will use Default::default() if the field is missing
    pub uploader: Option<WHUploader>,  // Make uploader optional
    pub views: i32,
    pub favorites: i32,
    pub source: String,
    pub purity: String,
    pub category: String,
    pub dimension_x: i32,
    pub dimension_y: i32,
    pub resolution: String,
    pub ratio: String,
    pub file_size: i32,
    pub file_type: String,
    pub created_at: String,
    pub colors: Vec<String>,
    pub path: String,
    pub thumbs: WHImageThumbnailImageData,
    #[serde(default)]
    pub tags: Option<Vec<WHTag>>,  // Make tags optional

}


#[derive(Debug, Deserialize, Clone)]
pub struct WHImageThumbnailImageData {
    large: String,
    original: String,
    pub small: String,
}