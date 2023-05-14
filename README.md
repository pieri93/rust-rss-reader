# Rust RSS Reader
This is a simple RSS reader implemented in Rust, using the `reqwest` and `rss` crates.

## Usage 

To use the RSS reader, simply run the program and enter the URL of the RSS feed you want to read when prompted:

```
$ cargo run
Enter RSS feed URL:
```
The program will then retrieve and parse the RSS feed, and display its contents to the console.

## Dependencies

The RSS reader depends on the following Rust crates:

- **reqwest:** For making HTTP requests to retrieve the RSS feed.
- **rss:** For parsing the RSS feed into a structured format.

These crates are specified in the Cargo.toml file and will be automatically downloaded and installed by Cargo when you run the program for the first time.

## Contributing

Contributions to the RSS reader are welcome! If you find a bug or have a suggestion for a new feature, please open an issue on GitHub.