use std::sync::Arc;

use crate::domain::DomainError;
use crate::domain::catagory::{Catagory, CatagoryRepository};

#[derive(Clone)]
pub struct CatagoryService {
    repo: Arc<dyn CatagoryRepository>,
}

impl CatagoryService {
    pub fn new(repo: Arc<dyn CatagoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, name: String) -> Result<i64, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("name is required".into()));
        }
        self.repo.create(name).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Catagory>, DomainError> {
        self.repo.get_by_id(id).await
    }

    pub async fn get_all_catagories(&self) -> Result<Vec<Catagory>, DomainError> {
        self.repo.get_all_catagories().await
    }

    pub async fn update(&self, id: i64, name: String) -> Result<Catagory, DomainError> {
        self.repo.update(id, name).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let catagory = self.repo.get_by_id(id).await?;
        if catagory.is_none() {
            return Err(DomainError::NotFound);
        }
        self.repo.delete(id).await
    }
}
