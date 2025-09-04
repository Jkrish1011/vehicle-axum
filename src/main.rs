use axum::{
    debug_handler, routing::{get, post}, Json, Router
};

use serde;

mod vehicle;

#[tokio::main]
async fn main() {
    // Create Axum Router
    let router01 = Router::new()
                            .route("/vehicle", get(vehicle::vehicle_get))
                            .route("/vehicle", post(vehicle::vehicle_post));

    // Define ID and port listeners
    let address = "0.0.0.0:6570";

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // axum serve to launch the webserver
    axum::serve(listener, router01).await.unwrap();
}
