pub mod linux;


pub fn change(path: &str) {
    // TODO: Find out what OS is running

    // for now just work with Linux Gnome Desktop Environment
    linux::gnome(path).expect("Linux - Gnome: Error changing background");
}