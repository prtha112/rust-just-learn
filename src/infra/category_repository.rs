use async_trait::async_trait;
use sqlx::PgPool;
use tracing::instrument;

use crate::domain::category::{Category, CategoryRepository};
use crate::domain::DomainError;

#[derive(Clone)]
pub struct PostgresCategoryRepository {
    pool: PgPool,
}

impl PostgresCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for PostgresCategoryRepository {
    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn create(&self, name: String) -> Result<i64, DomainError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO catagories (name)
            VALUES ($1)
            RETURNING id
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(row.id)
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn get_by_id(&self, id: i64) -> Result<Option<Category>, DomainError> {
        let row = sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, active
            FROM catagories
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
    async fn get_all_categories(&self) -> Result<Vec<Category>, DomainError> {
        let rows = sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, active
            FROM catagories
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(rows)
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn update(&self, id: i64, name: String) -> Result<Category, DomainError> {
        let row = sqlx::query_as!(
            Category,
            r#"
            UPDATE catagories
            SET name = $1
            WHERE id = $2
            RETURNING id, name, active
            "#,
            name,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(row)
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let _row = sqlx::query!(
            r#"
            DELETE FROM catagories
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
