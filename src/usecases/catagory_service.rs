use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::catagory::{Catagory, CatagoryRepository};
use crate::domain::user::DomainError;

#[derive(Clone)]
pub struct CatagoryService {
    catagory_repository: Arc<dyn CatagoryRepository>,
}

impl CatagoryService {
    pub fn new(catagory_repository: Arc<dyn CatagoryRepository>) -> Self {
        Self { catagory_repository }
    }
}

#[async_trait]
impl CatagoryRepository for CatagoryService {
    async fn create(&self, name: String) -> Result<i64, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("name is required".into()));
        }
        self.catagory_repository.create(name).await
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<Catagory>, DomainError> {
        self.catagory_repository.get_by_id(id).await
    }

    async fn get_all_catagories(&self) -> Result<Vec<Catagory>, DomainError> {
        self.catagory_repository.get_all_catagories().await
    }

    async fn update(&self, id: i64, name: String) -> Result<Catagory, DomainError> {
        self.catagory_repository.update(id, name).await
    }

    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let catagory = self.catagory_repository.get_by_id(id).await?;
        if catagory.is_none() {
            return Err(DomainError::NotFound);
        }
        self.catagory_repository.delete(id).await
    }
}