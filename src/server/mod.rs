use std::fmt::{self};
use std::{borrow::Cow, io::Read};

use rouille::{router, Request, Response};
use rusqlite::Connection;
use url_shortener::Url;

pub fn initialise_server() {
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/api/url/{location: String}) => {
                let conn = Connection::open("url_shortener.db").unwrap();
                let url = Url::fetch_from_db(&conn, location);
                display_url_json(url)
            },
            (GET) (/{location: String}) => {
                let conn = Connection::open("url_shortener.db").unwrap();
                let url = Url::fetch_from_db(&conn, location);
                display_url_redirect(url)
            },
            (POST) (/api/url) => {
                execute(&|| {
                    let conn = Connection::open("url_shortener.db").unwrap();
                    let mut url = get_url_request_body(request)?;
                    url.post_to_db(&conn)?;
                    Ok(created_response(&url))
                })
            },

            _ => rouille::Response::empty_404()
        )
    });
}

fn execute(f: &dyn Fn() -> Result<Response, Error>) -> Response {
    let result = f();
    result.unwrap_or_else(|e| internal_error_response(e.to_string()))
}

fn get_url_request_body(request: &Request) -> Result<Url, Error> {
    if let Some(mut request_body) = request.data() {
        let mut buf = String::new();
        request_body.read_to_string(&mut buf)?;
        let url: Url = serde_json::from_str(&buf)?;
        return Ok(url);
    }
    Err(Error::CustomError(String::from("request body missing")))
}

#[derive(Debug)]
enum Error {
    CustomError(String),
    RusqliteError(rusqlite::Error),
    IOError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::RusqliteError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CustomError(s) => write!(f, "{}", s),
            Error::RusqliteError(inner) => write!(f, "{}", inner),
            Error::IOError(inner) => write!(f, "{}", inner),
            Error::SerdeError(inner) => write!(f, "{}", inner),
        }
    }
}

fn display_url_helper(
    url: rusqlite::Result<Option<Url>>,
    on_existing_url: &dyn Fn(&Url) -> Response,
) -> Response {
    match url {
        Ok(Some(u)) => on_existing_url(&u),
        Ok(None) => Response::empty_404(),
        Err(e) => internal_error_response(e.to_string()),
    }
}

fn created_response(url: &Url) -> Response {
    let mut response = Response::json(&url);
    response.status_code = 201;
    response
}

fn internal_error_response(text: String) -> Response {
    let mut response = Response::text(format!("server error: {}", text));
    response.status_code = 500;
    response
}

fn display_url_json(url: rusqlite::Result<Option<Url>>) -> Response {
    display_url_helper(url, &|u| rouille::Response::json(&u))
}

fn display_url_redirect(url: rusqlite::Result<Option<Url>>) -> Response {
    display_url_helper(url, &|u| {
        let mut response = Response::text("OK");
        response.status_code = 301;
        response.headers = vec![(Cow::from("Location"), Cow::from(u.target().to_string()))];
        return response;
    })
}
