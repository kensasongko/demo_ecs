#[macro_use] extern crate rocket;

use rocket::response::{content, status};
use rocket::http::{Status, ContentType};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Responder)]
#[response(status = 418, content_type = "json")]
struct RawTeapotJson(&'static str);

#[get("/json")]
fn json() -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::JSON, "{ \"Hello\": \"world! v3\" }"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![json])
}
