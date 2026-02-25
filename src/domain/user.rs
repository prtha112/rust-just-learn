use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub active: bool,
}

impl User {
    pub fn greet(&self) -> String {
        format!("Hello {}", self.name)
    }
}

// --- Trait Speak (default method shout) ---
pub trait Speak {
    type Err;

    fn speak(&self) -> Result<String, Self::Err>;
    fn shout(&self) -> Result<String, Self::Err> {
        Ok(self.speak()?.to_uppercase())
    }
}

impl Speak for User {
    type Err = DomainError;

    fn speak(&self) -> Result<String, Self::Err> {
        Ok(format!("Hello {}", self.name))
    }
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("not found")]
    NotFound,
    #[error("unexpected error: {0}")]
    Unexpected(String),
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, name: String) -> Result<i64, DomainError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, DomainError>;
    async fn get_all(&self) -> Result<Vec<User>, DomainError>;
}