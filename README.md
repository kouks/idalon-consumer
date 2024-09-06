# Idalon Rust API client

## Installation

```sh
cargo add idalon_consumer
```

## About

This libary provides a 'Model' for each resource of the idalon.com API. Each
model has a generic set of helper methods, usually to fetch a detail of a
resource or to fetch a listing of the resource with different filters. It also
provides an automatic paginator in the form of a tokio stream.

## Examples

Fetching a single Night:

```rs
let night = Night::find_one("6c3aee7c-b34c-4c1b-80e5-0e4bec31a401").await;

assert!(night.captured_hydrolysts_count == 6);
```

Fetching a paginated listing:

```rs
let mut paginator = Night::paginate(NightFilters::leaderboard());

while let Some(result) = paginator.next().await {
    if result.is_err() {
        return Err(IdalonError::new("Failed to fetch page."));
    }

    let page = result.expect("Result is not None as checked before.");

    tokio::spawn(async move {
        for run in page.items {
            println!("{:?}", run)
        }

        // println!("Median is {:.3}", find_median(&page.items));
        // println!("Average is {:.3}", find_average(&page.items));
    });
}
```
