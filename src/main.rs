use core::panic;

use rusqlite::{Connection};
use url_shortener::Url;
use rouille::{Response, router};


fn main() {
    init();
    println!("Now listening on localhost:8000");
    rouille::start_server("localhost:8000", move |request| {
    
    router!(request,
        (GET) (/api/url/{location: String}) => {
            let conn = Connection::open("url_shortener.db").unwrap();

            match Url::fetch_from_db(&conn, location) {
                Ok(Some(u)) => rouille::Response::json(&u),
                Ok(None) => Response::empty_404(),
                Err(e) => {
                    println!("error: {}", e.to_string());
                    let mut response = Response::text("server error");
                    response.status_code = 500;
                    return response;
                }
            }     
        },

        _ => rouille::Response::empty_404()
    )
});
}

fn init() {
    let conn = Connection::open("url_shortener.db");
    if let Err(e) = conn {
        panic!("{}", e.to_string());
    }
    let conn = conn.unwrap();

    if let Err(e) = conn.execute(
        "CREATE TABLE IF NOT EXISTS url (
                  location        TEXT PRIMARY KEY,
                  target          TEXT NOT NULL,
                  created_at      TEXT
                  )",
        [],
    ) {
        panic!("{}", e.to_string());
    }
}
