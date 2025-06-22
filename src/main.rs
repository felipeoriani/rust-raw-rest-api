mod db;
mod http;
mod model;

use db::set_database;
use dotenv::dotenv;
use http::init_server;
use std::env;

#[macro_use]
extern crate serde_derive;

fn main() {
    dotenv().ok();

    if let Err(e) = set_database() {
        println!("Error: {}", e);
        return;
    }

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<i32>()
        .unwrap();

    init_server(port);
}
