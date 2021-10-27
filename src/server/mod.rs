use std::{borrow::Cow};

use rouille::{Response, router};
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

            _ => rouille::Response::empty_404()
        )
    });
}

fn display_url_helper(url: rusqlite::Result<Option<Url>>, on_existing_url: &dyn Fn(&Url) -> Response) -> Response {
    match url {
        Ok(Some(u)) => on_existing_url(&u),
        Ok(None) => Response::empty_404(),
        Err(e) => {
            println!("error: {}", e.to_string());
            let mut response = Response::text("server error");
            response.status_code = 500;
            return response;
        }
    }
}

fn display_url_json(url: rusqlite::Result<Option<Url>>) -> Response {
    display_url_helper(url, &|u| rouille::Response::json(&u))
}

fn display_url_redirect(url: rusqlite::Result<Option<Url>>) -> Response {
    display_url_helper(url, &|u| {
        let mut response = Response::text("OK");
            response.status_code = 301;
            response.headers = vec![(Cow::from("Location"), Cow::from(u.target().to_string()))];
            // match &response.headers[0].0 {
            //     Cow::Borrowed(s) => println!("borrowed {}", s),
            //     Cow::Owned(s) => println!("owned {}", s)
            // };
            // match &response.headers[0].1 {
            //     Cow::Borrowed(s) => println!("borrowed {}", s),
            //     Cow::Owned(s) => println!("owned {}", s)
            // };
            
            // println!("{:?}", response.headers);
            return response;    
    })
}
