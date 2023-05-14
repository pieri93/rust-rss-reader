use rss::Channel;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut url = String::new();

    // Ask user's url as input
    println!("Enter RSS feed URL: ");
    io::stdin().read_line(&mut url)?;

    let url = url.trim();

    let resp = reqwest::blocking::get(url)?;
    let body = resp.bytes()?;
    let channel = Channel::read_from(&body[..])?;

    println!("Title: {}", channel.title());
    println!("Link: {}", channel.link());
    println!("Description: {}", channel.description());

    for item in channel.items() {
        println!("Title: {}", item.title().unwrap_or_default());
        println!("Link: {}", item.link().unwrap_or_default());
        println!("Description: {}", item.description().unwrap_or_default());
        println!("Published: {}", item.pub_date().unwrap_or_default());
        println!();
    }

    println!("End");
    Ok(())
}
