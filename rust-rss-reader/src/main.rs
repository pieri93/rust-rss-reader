use rss::Channel;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let mut url = String::new();

    // Ask user's url as input
    println!("Enter RSS feed URL: ");
    io::stdin().read_line(&mut url)?;

    let url = url.trim();

    let resp = reqwest::blocking::get(url)?;
    let body = resp.bytes()?;
    let channel = Channel::read_from(&body[..])?;

    // Create or open the output file
    let file_path = Path::new("rss_output.txt");
    let mut file = File::create(file_path)?;

    // Redirect all prints to the output file
    for item in channel.items() {
        writeln!(file, "Title: {}", item.title().unwrap_or_default());
        writeln!(file, "Link: {}", item.link().unwrap_or_default());
        writeln!(
            file,
            "Description: {}",
            item.description().unwrap_or_default()
        );
        writeln!(file, "Published: {}", item.pub_date().unwrap_or_default());
        writeln!(file, "\n");
    }

    println!("End of ress reading");
    Ok(())
}
