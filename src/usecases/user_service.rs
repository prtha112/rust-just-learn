use std::sync::Arc;
use crate::domain::user::{DomainError, User, UserRepository};

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, username: String, password: String) -> Result<i64, DomainError> {
        if username.trim().is_empty() {
            return Err(DomainError::Validation("username is required".into()));
        }
        if password.trim().is_empty() {
            return Err(DomainError::Validation("password is required".into()));
        }
        self.repo.create(username, password).await
    }

    pub async fn get_user(&self, id: i64) -> Result<User, DomainError> {
        let user = self.repo.get_by_id(id).await?;
        user.ok_or(DomainError::NotFound)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, DomainError> {
        let users = self.repo.get_all_users().await?;
        Ok(users)
    }

    pub async fn update_user(&self, id: i64, username: String, password: String) -> Result<User, DomainError> {
        if username.trim().is_empty() {
            return Err(DomainError::Validation("username is required".into()));
        }
        if password.trim().is_empty() {
            return Err(DomainError::Validation("password is required".into()));
        }
        self.repo.update(id, username, password).await
    }
}