use async_trait::async_trait;
use sqlx::PgPool;
use tracing::instrument;

use crate::domain::product::{Product, ProductRepository};
use crate::domain::DomainError;

#[derive(Clone)]
pub struct PostgresProductRepository {
    pool: PgPool,
}

impl PostgresProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn create(&self, product: Product) -> Result<i64, DomainError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO products (name, description, price, stock, category_id, active)
            VALUES ($1, $2, $3::float8, $4, $5, $6)
            RETURNING id
            "#,
            product.name,
            product.description,
            product.price,
            product.stock,
            product.category_id,
            product.active
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(row.id)
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn get_by_product_id(&self, id: i64) -> Result<Option<Product>, DomainError> {
        let row = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, description, price::float8 as "price!", stock, category_id, active
            FROM products
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(row) 
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn get_by_category_id(&self, category_id: i64) -> Result<Vec<Product>, DomainError> {
        let rows = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, description, price::float8 as "price!", stock, category_id, active
            FROM products
            WHERE category_id = $1
            "#,
            category_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(rows)
    }
}
