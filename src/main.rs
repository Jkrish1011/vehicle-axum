use axum::{
    debug_handler, routing::{get, post}, Json, Router
};

use serde;


#[tokio::main]
async fn main() {
    // Create Axum Router
    let router01 = Router::new()
                            .route("/vehicle", get(vehicle_get))
                            .route("/vehicle", post(vehicle_post));

    // Define ID and port listeners
    let address = "0.0.0.0:6570";

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // axum serve to launch the webserver
    axum::serve(listener, router01).await.unwrap();
}

#[derive(Debug, serde::Serialize)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: String
}

#[debug_handler]
async fn vehicle_get() -> Json<Vehicle> {
    Json::from(Vehicle {
        manufacturer: "Dodge".to_string(),
        model: "RRA 12".to_string(),
        year: 1995,
        id: uuid::Uuid::new_v4().to_string()
    })
}

async fn vehicle_post() {

}
