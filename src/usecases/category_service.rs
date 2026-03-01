use std::sync::Arc;

use crate::domain::DomainError;
use crate::domain::category::{Category, CategoryRepository};

#[derive(Clone)]
pub struct CategoryService {
    repo: Arc<dyn CategoryRepository>,
}

impl CategoryService {
    pub fn new(repo: Arc<dyn CategoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, name: String) -> Result<i64, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("name is required".into()));
        }
        self.repo.create(name).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Category>, DomainError> {
        self.repo.get_by_id(id).await
    }

    pub async fn get_all_categories(&self) -> Result<Vec<Category>, DomainError> {
        self.repo.get_all_categories().await
    }

    pub async fn update(&self, id: i64, name: String) -> Result<Category, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("name is required".into()));
        }
        if self.repo.get_by_id(id).await?.is_none() {
            return Err(DomainError::NotFound);
        }
        self.repo.update(id, name).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let category = self.repo.get_by_id(id).await?;
        if category.is_none() {
            return Err(DomainError::NotFound);
        }
        self.repo.delete(id).await
    }
}
