#[macro_use]
extern crate rocket;

use shuttle_service::ShuttleRocket;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn init() -> ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![hello]);

    Ok(rocket)
}