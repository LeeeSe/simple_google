use clap::{Arg, Command};
use simple_google::search_and_parse;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("simple_google")
        .version("0.1.0")
        .about("A simple Google search tool")
        .arg(
            Arg::new("query")
                .help("Search query")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("pages")
                .short('p')
                .long("pages")
                .help("Number of pages to fetch")
                .value_name("NUM")
                .default_value("1"),
        )
        .get_matches();

    let query = matches.get_one::<String>("query").unwrap();
    let pages: u32 = matches
        .get_one::<String>("pages")
        .unwrap()
        .parse()
        .unwrap_or(1);

    let results = search_and_parse(query, pages).await?;

    for (i, result) in results.iter().enumerate() {
        println!("\n--- Result {} ---", i + 1);
        println!("Title: {}", result.title);
        println!("Link: {}", result.link);
        println!("Source: {}", result.source);
        println!("Snippet: {}", result.snippet);
    }

    Ok(())
}
