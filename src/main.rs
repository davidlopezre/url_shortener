#![allow(unreachable_code)]
use rouille;
use rouille::router;

use std::iter::repeat_with;
fn main() {
    println!("Now listening on localhost:8000");
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/{hash: String}) => {
                rouille::Response::redirect_303("https://www.rust-lang.org/")
            },

            (GET) (/api/urls/{hash: String}) => {
                rouille::Response::empty_404()
            },

            (POST) (/api/urls) => {
                let s: String = repeat_with(fastrand::alphanumeric).take(10).collect();
                println!("{}", s);
                rouille::Response::empty_404()
            },

            // The code block is called if none of the other blocks matches the request.
            // We return an empty response with a 404 status code.
            _ => rouille::Response::empty_404()
        )
    });
}