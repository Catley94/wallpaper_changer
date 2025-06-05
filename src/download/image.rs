use std::error::Error;
use std::fs::File;
use std::io::{copy, Cursor, Read};
use std::path::Path;
use crate::models::wallhaven::WHImageData;
use crate::utils;

pub fn thumbnail(image: &&WHImageData, local_path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let file_path = format!("{}/wallhaven-{}.{}", local_path, &image.id, utils::get_file_extension(&image.file_type));

    // Check if file already exists
    if Path::new(&file_path).exists() {
        println!("File {} already exists, skipping download", file_path);
        return Ok(());
    }
    println!("Image ID: {}", image.id);
    println!("Image URL: {}", image.url);
    println!("Image path: {}", image.path);
    println!("Image thumbs: {}", image.thumbs.small);

    // Create the output file
    let mut image_file = File::create(file_path).expect("Failed to create file");

    let image_request = ureq::get(&image.thumbs.small)
        .call()?;

    let (_, body) = image_request.into_parts();

    let mut bytes_buf: Vec<u8> = Vec::with_capacity(1000);

    body.into_reader()
        .read_to_end(&mut bytes_buf)?;

    // // Copy the response body to the file
    copy(&mut Cursor::new(bytes_buf), &mut image_file)?;
    Ok(())
}

pub fn original(image: &WHImageData, local_path: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let file_path = format!("{}/wallhaven-{}.{}", local_path, &image.id, utils::get_file_extension(&image.file_type));

    println!("Original: Image path: {}", file_path);

    // Check if file already exists
    if Path::new(&file_path).exists() {
        println!("File {} already exists, skipping download", file_path);
        return Ok(file_path);
    }
    println!("Image ID: {}", image.id);
    println!("Image URL: {}", image.url);
    println!("Image path: {}", image.path);
    println!("Image thumbs: {}", image.thumbs.small);

    // Create the output file
    let mut image_file = File::create(&file_path).expect("Failed to create file");

    let image_request = ureq::get(&image.path)
        .call()?;

    let (_, body) = image_request.into_parts();

    let mut bytes_buf: Vec<u8> = Vec::with_capacity(1000);

    body.into_reader()
        .read_to_end(&mut bytes_buf)?;

    // // Copy the response body to the file
    copy(&mut Cursor::new(bytes_buf), &mut image_file)?;
    Ok(file_path)
}