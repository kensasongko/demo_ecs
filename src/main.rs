#![feature(proc_macro_hygiene, decl_macro)]

use std::str::FromStr;

use rocket::{get, routes};
use rocket_cors as cors;
use crate::cors::{AllowedHeaders, AllOrSome, CorsOptions};

#[get("/")]
fn cors<'a>() -> &'a str {
    "{\"message\": \"Hello from Rocket!\"}"
}


fn main() {
    let cors = CorsOptions {
        allowed_origins: AllOrSome::All,
        allowed_methods: ["Get"]
           .iter()
           .map(|s| FromStr::from_str(s).unwrap())
           .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: false,
        expose_headers: ["Content-Type", "application/json"]
            .iter()
            .map(ToString::to_string)
            .collect(),
        ..Default::default()
    }
    .to_cors();

    rocket::ignite()
        .mount("/", routes![cors])
        .manage(cors)
        .launch();
}
