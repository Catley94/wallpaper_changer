pub mod flags;
pub mod os;

pub fn get_file_extension(file_type: &str) -> &str {
    match file_type.to_lowercase().as_str() {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/webp" => "webp",
        _ => "jpg"  // default to jpg if unknown
    }
}
