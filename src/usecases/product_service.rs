use std::sync::Arc;

use crate::domain::DomainError;
use crate::domain::product::{Product, ProductRepository};

#[derive(Clone)]
pub struct ProductService {
    repo: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(repo: Arc<dyn ProductRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, product: Product) -> Result<i64, DomainError> {
        if product.name.trim().is_empty() {
            return Err(DomainError::Validation("name is required".into()));
        }
        self.repo.create(product).await
    }

    pub async fn get_by_product_id(&self, id: i64) -> Result<Option<Product>, DomainError> {
        self.repo.get_by_product_id(id).await
    }

    pub async fn get_by_category_id(&self, category_id: i64) -> Result<Vec<Product>, DomainError> {
        self.repo.get_by_category_id(category_id).await
    }
}
