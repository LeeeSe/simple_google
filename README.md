# Simple Google

A Rust library and CLI tool for Google search that bypasses CAPTCHA verification.

## Installation

```bash
cargo install --git https://github.com/LeeeSe/simple_google
```

## CLI Usage

```bash
# Basic search
simple_google "rust programming"

# Multiple pages
simple_google "machine learning" --pages 3
```

## Library Usage

```rust
use simple_google::{search_and_parse, SearchResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query = "rust programming language";
    let pages = 1;
    
    let results = search_and_parse(query, pages).await?;
    
    for result in results {
        println!("Title: {}", result.title);
        println!("URL: {}", result.link);
        println!("Snippet: {}", result.snippet);
    }
    
    Ok(())
}
```