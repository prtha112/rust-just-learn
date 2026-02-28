use async_trait::async_trait;

use crate::domain::DomainError;

#[async_trait]
pub trait CatagoryRepository: Send + Sync {
    async fn create(&self, name: String) -> Result<i64, DomainError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<Catagory>, DomainError>;
    async fn get_all_catagories(&self) -> Result<Vec<Catagory>, DomainError>;
    async fn update(&self, id: i64, name: String) -> Result<Catagory, DomainError>;
    async fn delete(&self, id: i64) -> Result<(), DomainError>;
}

#[derive(Debug)]
pub struct Catagory {
    pub id: i64,
    pub name: String,
    pub active: bool,
}