# URL Shortener

A simple URL shortener written in Rust. 

# Requirements
* rust, cargo (tested with 1.56.0 (09c42c458 2021-10-18))
* sqlite3 (tested with 3.31.1)

# How to Run

```
cargo run
```

# Usage

## Url Object Example
```
{
    "location":"test_location_2",
    "target":"https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    "created_at":"2021-06-08T11:29:11.124Z"
}
```

## Endpoints
*GET /api/url/{location}*: Returns a URL object in JSON format

*GET /{location}*: Redirects to configured target
