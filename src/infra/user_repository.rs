use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::user::{DomainError, User, UserRepository};

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, name: String) -> Result<i64, DomainError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO users (name, active)
            VALUES ($1, TRUE)
            RETURNING id
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(row.id)
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as!(User, 
            r#"
            SELECT id, name, active
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.id,
            name: r.name,
            active: r.active,
        }))
    }
}
