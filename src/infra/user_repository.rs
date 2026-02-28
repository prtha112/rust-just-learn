use async_trait::async_trait;
use sqlx::PgPool;
use tracing::instrument;

use crate::domain::user::{DomainError, User, UserRepository};

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn mask_password(password: String) -> String {
        let mut masked_password = String::new();
        for _ in 0..password.len() {
            masked_password.push('*');
        }
        masked_password
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn create(&self, username: String, password: String) -> Result<i64, DomainError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO users (username, password, active)
            VALUES ($1, $2, TRUE)
            RETURNING id
            "#,
            username,
            password
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

        Ok(row.map(|r| User {
            id: r.id,
            username: r.username,
            password: Self::mask_password(r.password),
            active: r.active,
        }))
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

        Ok(rows.into_iter().map(|r| User {
            id: r.id,
            username: r.username,
            password: Self::mask_password(r.password),
            active: r.active,
        }).collect())
    }

    #[instrument(skip(self), err, fields(db = "postgres"))]
    async fn update(&self, id: i64, username: String, password: String) -> Result<User, DomainError> {
        let row = sqlx::query_as!(User, 
            r#"
            UPDATE users 
            SET username = $1, password = $2 
            WHERE id = $3 
            RETURNING id, username, password, active
            "#,
            username,
            password,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(User {
            id: row.id,
            username: row.username,
            password: Self::mask_password(row.password),
            active: row.active,
        })
    }
}
