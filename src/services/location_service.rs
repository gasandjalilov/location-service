use mongodm::bson::Bson;
use crate::dtos::location_dto::LocationDataDTO;
use crate::repositories::location_repository::LocationRepository;

#[derive(Clone, Debug)]
pub struct LocationService{
    repository: LocationRepository
}

impl LocationService{
    pub fn new(repository: LocationRepository) -> Self{
        Self{
            repository
        }
    }

    pub async fn save_location(&self, location_data_dto: LocationDataDTO) -> Result<Bson, String> {
        self.repository.save_location(location_data_dto).await
            .map(|bson| bson)
            .map_err(|err| err.to_string())
    }
}