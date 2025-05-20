use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationDTO {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationDataDTO {
    pub client_id: i32,
    pub location: LocationDTO,
    pub session_id: String
}