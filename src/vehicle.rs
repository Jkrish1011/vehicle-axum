use axum::{
    debug_handler, Json
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>
}

#[debug_handler]
pub async fn vehicle_get() -> Json<Vehicle> {
    Json::from(Vehicle {
        manufacturer: "Dodge".to_string(),
        model: "RRA 12".to_string(),
        year: 1995,
        id: Some(uuid::Uuid::new_v4().to_string())
    })
}

#[debug_handler]
pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> Json<Vehicle> {
    
    v.id = Some(uuid::Uuid::new_v4().to_string());

    println!("Manufacturer: {0}, model: {1}, year: {2}", v.manufacturer, v.model, v.year);

    Json::from(v)
}
