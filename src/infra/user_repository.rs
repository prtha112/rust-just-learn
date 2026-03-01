use async_trait::async_trait;
use sqlx::PgPool;
use tracing::instrument;

use crate::domain::DomainError;
use crate::domain::user::{User, UserRepository};
use crate::infra::crypto::hash_password;

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
    #[instrument(skip(self, password), err, fields(db = "postgres"))]
    async fn create(&self, username: String, password: String) -> Result<i64, DomainError> {
        let hashed = hash_password(password).await?;

        let row = sqlx::query!(
            r#"
            INSERT INTO users (username, password, active)
            VALUES ($1, $2, TRUE)
            RETURNING id
            "#,
            username,
            hashed
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(row.id)
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as!(User,
            r#"
            SELECT id, username, password, active
            FROM users
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
    async fn get_by_username(&self, username: String) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as!(User,
            r#"
            SELECT id, username, password, active
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(row)
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn get_all_users(&self) -> Result<Vec<User>, DomainError> {
        let rows = sqlx::query_as!(User,
            r#"
            SELECT id, username, password, active
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(rows)
    }

    #[instrument(skip(self, password), err, fields(db = "postgres"))]
    async fn update(&self, id: i64, username: String, password: String) -> Result<User, DomainError> {
        let hashed = hash_password(password).await?;

        let row = sqlx::query_as!(User,
            r#"
            UPDATE users
            SET username = $1, password = $2
            WHERE id = $3
            RETURNING id, username, password, active
            "#,
            username,
            hashed,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(row)
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            DELETE FROM users
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
