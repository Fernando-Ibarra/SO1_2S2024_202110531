#[macro_use]
extern crate rocket;

use main::athlete_client::AthleteClient;
use main::AthleteRequest;

pub mod main {
    tonic::include_proto!("main");
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World! From Actix Web Framework and Rust"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}

#[tokio::main]
async fn grpc_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AthleteClient::connect("http://[::1]:3000").await?;
    let request = tonic::Request::new(AthleteRequest {
        student: "John Doe".into(),
        age: 25,
        faculty: "Computer Science".into(),
        distance: 1000,
    });

    let response = client.get_athlete(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}