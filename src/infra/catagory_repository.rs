use async_trait::async_trait;
use sqlx::PgPool;
use tracing::instrument;

use crate::domain::catagory::{Catagory, CatagoryRepository};
use crate::domain::user::DomainError;

#[derive(Clone)]
pub struct PostgresCatagoryRepository {
    pool: PgPool,
}

impl PostgresCatagoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CatagoryRepository for PostgresCatagoryRepository {
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
    async fn get_by_id(&self, id: i64) -> Result<Option<Catagory>, DomainError> {
        let row = sqlx::query!(
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
        Ok(row.map(|r| Catagory { id: r.id, name: r.name, active: r.active }))
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn get_all_catagories(&self) -> Result<Vec<Catagory>, DomainError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, active 
            FROM catagories
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;
        Ok(rows.into_iter().map(|r| Catagory { id: r.id, name: r.name, active: r.active }).collect())
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn update(&self, id: i64, name: String) -> Result<Catagory, DomainError> {
        let row = sqlx::query!(
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
        Ok(Catagory { id: row.id, name: row.name, active: row.active })
    }
}