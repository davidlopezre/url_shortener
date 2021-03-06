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

### Get URL in JSON format

*GET /api/url/{location}*

### Redirect to URL 

*GET /{location}*

Redirects to configured target

### Create a new URL

*POST /api/url*

Sample body
```
{
    "location":"test_location_2",
    "target":"https://www.youtube.com/watch?v=dQw4w9WgXcQ",
}
```
