use std::sync::Arc;
use crate::domain::user::{DomainError, User, UserRepository};

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, name: String) -> Result<i64, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("name is required".into()));
        }
        self.repo.create(name).await
    }

    pub async fn get_user(&self, id: i64) -> Result<User, DomainError> {
        let user = self.repo.get_by_id(id).await?;
        user.ok_or(DomainError::NotFound)
    }
}