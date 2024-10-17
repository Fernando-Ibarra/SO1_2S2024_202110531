#[macro_use]
extern crate rocket;

use rocket::response::content;
// use tonic::transport::Channel;
use main::athleteuide_client::AthleteuideClient;
use main::AthleteRequest;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use tokio::task;

#[derive(Deserialize, Serialize)]
struct HttpAthleteRequest {
    student: String,
    age: i64,
    faculty: String,
    discipline: i64,
}

pub mod main {
    tonic::include_proto!("main");
}


#[post("/", format = "json", data = "<athlete>")]
async fn create_athlete(athlete: Json<HttpAthleteRequest>) -> content::RawHtml<String> {
    let athlete_data = athlete.into_inner();
    let grpc_response = match athlete_data.discipline {
        1 => task::spawn(async move { grpc_swim_server(athlete_data).await }).await.unwrap(),
        2 => task::spawn(async move { grpc_run_server(athlete_data).await }).await.unwrap(),
        3 => task::spawn(async move { grpc_box_server(athlete_data).await }).await.unwrap(),
        _ => Err("Invalid discipline".into()),
    };

    match grpc_response {
        Ok(response) => content::RawHtml(format!("gRPC response: {}", response)),
        Err(e) => content::RawHtml(format!("Error: {}", e)),
    }
}

async fn grpc_swim_server(athlete: HttpAthleteRequest) -> Result<String, String> {
    let mut client = match AthleteuideClient::connect("http://go-service-swim:3001").await {
        Ok(client) => client,
        Err(e) => return Err(format!("Failed to connect to gRPC server Swim: {}", e)),
    };

    // Crear la solicitud para el servidor gRPC
    let request = tonic::Request::new(AthleteRequest {
        student: athlete.student,
        age: athlete.age,
        faculty: athlete.faculty,
        discipline: athlete.discipline,
    });

    // Enviar la solicitud gRPC y obtener la respuesta
    match client.create_athlete(request).await {
        Ok(response) => Ok(response.into_inner().student),
        Err(e) => Err(format!("Error sending request to gRPC server 1: {}", e)),
    }
}

async fn grpc_run_server(athlete: HttpAthleteRequest) -> Result<String, String> {
    let mut client = match AthleteuideClient::connect("http://go-service-run:3002").await {
        Ok(client) => client,
        Err(e) => return Err(format!("Failed to connect to gRPC server Run: {}", e)),
    };

    let request = tonic::Request::new(AthleteRequest {
        student: athlete.student,
        age: athlete.age,
        faculty: athlete.faculty,
        discipline: athlete.discipline,
    });

    match client.create_athlete(request).await {
        Ok(response) => Ok(response.into_inner().student),
        Err(e) => Err(format!("Error sending request to gRPC server 2: {}", e)),
    }
}

async fn grpc_box_server(athlete: HttpAthleteRequest) -> Result<String, String> {
    let mut client = match AthleteuideClient::connect("http://go-service-box:3003").await {
        Ok(client) => client,
        Err(e) => return Err(format!("Failed to connect to gRPC server Run: {}", e)),
    };

    let request = tonic::Request::new(AthleteRequest {
        student: athlete.student,
        age: athlete.age,
        faculty: athlete.faculty,
        discipline: athlete.discipline,
    });

    match client.create_athlete(request).await {
        Ok(response) => Ok(response.into_inner().student),
        Err(e) => Err(format!("Error sending request to gRPC server 2: {}", e)),
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_athlete])
}