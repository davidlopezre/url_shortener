use core::panic;

use rusqlite::{Connection};
use url_shortener::Url;
use rouille::{Response, router};
use url_shortener::config;

fn main() {
    let cfg = config::Config::new();
    config::initialise_app(&cfg).unwrap_or_else(|e|panic!("failed to initialise_app: {}", e));

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
