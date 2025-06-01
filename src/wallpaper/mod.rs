use std::path::Path;
use std::process::Output;

pub mod linux;


pub fn change(path: &str) -> std::io::Result<Output> {
    // TODO: Find out what OS is running

    if !Path::new(&path).exists() {
        eprintln!("File does not exist {}", &path);
        panic!("File {} does not exist", &path);
    }
    // for now just work with Linux Gnome Desktop Environment
    linux::gnome(path)
}

// pub fn change_by_id(id: &str) -> std::io::Result<Output> {
//
//     // TODO:
//
//     let path: &str = ""; // TODO: Populate from id
//
//     change(path);
//
// }