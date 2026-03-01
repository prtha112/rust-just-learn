use async_trait::async_trait;

use crate::domain::DomainError;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub active: bool,
}

impl User {
    pub fn greet(&self) -> String {
        format!("Hello {}", self.username)
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
        Ok(format!("Hello {}", self.username))
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, username: String, password: String) -> Result<i64, DomainError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, DomainError>;
    async fn get_by_username(&self, username: String) -> Result<Option<User>, DomainError>;
    async fn get_all_users(&self) -> Result<Vec<User>, DomainError>;
    async fn update(&self, id: i64, username: String, password: String) -> Result<User, DomainError>;
    async fn delete(&self, id: i64) -> Result<(), DomainError>;
}
