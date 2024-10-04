#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, World! From Actix Web Framework and Rust"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}