use simple_google::{search_and_parse, SearchResult};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query = "rust programming language";
    let pages = 1;

    match search_and_parse(query, pages).await {
        Ok(results) => {
            println!("Found {} search results:", results.len());
            for (i, result) in results.iter().enumerate() {
                println!("\n{}: {}", i + 1, result.title);
                println!("URL: {}", result.link);
                println!("Source: {}", result.source);
                println!("Snippet: {}", result.snippet);
            }
        }
        Err(e) => eprintln!("Search failed: {}", e),
    }

    Ok(())
}