use mongodm::bson::Bson;
use mongodm::mongo::{Database, error::Error as MongoError};
use mongodm::ToRepository;
use crate::dtos::location_dto::LocationDataDTO;
use crate::exceptions::api_error::ApiError;
use crate::models::location::Location;

#[derive(Clone, Debug)]
pub struct LocationRepository {
    pub db: Database
}

impl LocationRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn save_location(&self, location: LocationDataDTO) -> Result<Bson, ApiError> {
        let location = Location::new(location);
        let repo = &self.db.repository::<Location>();
        repo.insert_one(location).await
            .map(|data| data.inserted_id)
            .map_err(ApiError::from)
    }
}