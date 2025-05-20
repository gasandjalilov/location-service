use mongodm::bson::Bson;
use mongodm::mongo::Database;
use mongodm::ToRepository;
use crate::dtos::location_dto::LocationDataDTO;
use crate::models::location::Location;

#[derive(Clone, Debug)]
pub struct LocationRepository {
    pub db: Database
}

impl LocationRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn save_location(&self, location: LocationDataDTO) -> Result<Bson, String> {
        let location = Location::new(location);
        let repo = &self.db.repository::<Location>();
        repo.insert_one(location).await
            .map(|data| data.inserted_id)
            .map_err(|e| e.to_string())
    }
}