use crate::utils::flags;
pub fn display_help_information(args: Vec<String>) {
    println!("wallpaper-changer v{}", env!("CARGO_PKG_VERSION"));
    println!("wallpaper-changer is a simple tool to create .desktop files for Linux. \n\
    By default it will open a GUI app, however this can also run in Terminal by specifying the below flags/arguments");
    println!("Usage: {}  [--global | --local] etc.", args[0]);
    println!("Options:");
    println!("  {}", flags::CHANGE_WALLPAPER);
    println!("      Change wallpaper providing an <id> or <url>");
    println!("  {}", flags::TOPIC);
    println!("      Search and download thumbnail pictures from a topic, if {} is not specified it will default to page 1.", flags::PAGE);
    println!("  {}", flags::PAGE);
    println!("      (Requires {}) Specify a page number to search for a topic", flags::TOPIC);
    println!("  {}", flags::VERSION);
    println!("      Show version information");
    println!("  {}", flags::HELP);
    println!("      Show this help message");
}