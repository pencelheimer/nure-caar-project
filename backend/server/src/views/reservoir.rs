use serde::{
    Deserialize, //
    Serialize,   //
};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateReservoirRequest {
    #[schema(example = "Main Water Tank")]
    pub name: String,
    #[schema(example = "Located on the roof")]
    pub description: Option<String>,
    #[schema(example = 1000.0)]
    pub capacity: f64,
    #[schema(example = "Roof Sector A")]
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateReservoirRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub capacity: Option<f64>,
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ReservoirResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub capacity: f64,
    pub location: Option<String>,
}
