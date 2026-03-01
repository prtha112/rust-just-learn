use std::sync::Arc;
use crate::domain::DomainError;
use crate::domain::user::{User, UserRepository};
use crate::infra::crypto::verify_password;

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
        self.repo.get_all_users().await
    }

    pub async fn update_user(&self, id: i64, username: String, password: String) -> Result<User, DomainError> {
        if username.trim().is_empty() {
            return Err(DomainError::Validation("username is required".into()));
        }
        if password.trim().is_empty() {
            return Err(DomainError::Validation("password is required".into()));
        }
        if self.repo.get_by_id(id).await?.is_none() {
            return Err(DomainError::NotFound);
        }
        self.repo.update(id, username, password).await
    }

    pub async fn delete_user(&self, id: i64) -> Result<(), DomainError> {
        let user = self.repo.get_by_id(id).await?;
        if user.is_none() {
            return Err(DomainError::NotFound);
        }
        self.repo.delete(id).await
    }

    pub async fn login(&self, username: String, password: String) -> Result<User, DomainError> {
        let user = match self.repo.get_by_username(username).await? {
            Some(u) => u,
            None => return Err(DomainError::Unauthorized),
        };

        if !verify_password(password, user.password.clone()).await? {
            return Err(DomainError::Unauthorized);
        }
        Ok(user)
    }
}