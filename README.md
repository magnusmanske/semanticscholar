# semanticscholar

A Rust wrapper around the [Semantic Scholar](https://www.semanticscholar.org/) API.

Also on [crates.io](https://crates.io/crates/semanticscholar).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
semanticscholar = "0.2"
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use semanticscholar::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Look up a paper by DOI
    let work = client.work("10.1038/nrn3241").await.unwrap();

    println!("Title: {:?}", work.title);
    println!("Authors: {}", work.authors.len());
    println!("Year: {:?}", work.year);

    for author in &work.authors {
        println!("  - {:?}", author.name);
    }
}
```

## Types

| Type | Description |
|------|-------------|
| `Client` | HTTP client for the Semantic Scholar API. Reuses connections across requests. |
| `Work` | A paper, including metadata, authors, topics, citations, and references. |
| `Author` | A paper author with optional ID, name, and URL. |
| `Topic` | A topic associated with a paper. |
| `Error` | Error enum covering invalid JSON, API errors, and HTTP failures. |

Papers can be looked up by DOI, arXiv ID, or Semantic Scholar paper ID.

## Error Handling

All API calls return `Result<_, semanticscholar::Error>`. The error type has three variants:

- `Error::Http` — network or connection failure
- `Error::Api` — the API returned an error (e.g. paper not found)
- `Error::InvalidJson` — the response could not be parsed

## License

MIT