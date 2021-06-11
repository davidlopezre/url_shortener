use core::panic;

use rusqlite::{Connection};
use url_shortener::Url;
use rouille::{Response, router};


fn main() {
    println!("Now listening on localhost:8000");
   rouille::start_server("localhost:8000", move |request| {
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
    router!(request,
        (GET) (/api/url/{location: String}) => {
            println!("u32 {:?}", location);

            match Url::fetch_from_db(&conn, location) {
                Ok(Some(u)) => rouille::Response::json(&u),
                Ok(None) => Response::empty_404(),
                Err(e) => {
                    let mut response = Response::text(e.to_string());
                    response.status_code = 400;
                    return response;
                }
            }     
        },

        // The code block is called if none of the other blocks matches the request.
        // We return an empty response with a 404 status code.
        _ => rouille::Response::empty_404()
    )
});
}
