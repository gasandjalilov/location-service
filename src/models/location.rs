use mongodm::{field, CollectionConfig, Index, IndexOption, Indexes, Model};
use mongodm::prelude::ObjectId;
use serde::{Deserialize, Serialize};

use crate::dtos::location_dto::LocationDataDTO;

pub struct LocationCollConf;

impl CollectionConfig for LocationCollConf {
    fn collection_name() -> &'static str {
        "location"
    }

    fn indexes() -> Indexes {
        Indexes::new()
            .with(Index::new("session_id").with_option(IndexOption::Unique))
            .with(Index::new(field!(client_id in Location)))
    }
}


#[derive(Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub client_id: i32,
    pub session_id: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Model for Location {
    type CollConf = LocationCollConf;
}

impl Location {
    pub fn new(location_dto: LocationDataDTO) -> Self {
        Location {
            id: None,
            client_id: location_dto.client_id,
            session_id: location_dto.session_id,
            latitude: location_dto.location.latitude,
            longitude: location_dto.location.longitude,
        }
    }
}