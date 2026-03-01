use async_trait::async_trait;

use crate::domain::DomainError;

#[derive(Debug)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category_id: i64,
    pub active: bool,
}

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, product: Product) -> Result<i64, DomainError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<Product>, DomainError>;
    async fn get_by_category_id(&self, category_id: i64) -> Result<Vec<Product>, DomainError>;
}