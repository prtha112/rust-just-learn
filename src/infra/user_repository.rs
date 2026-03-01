use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::instrument;

use crate::domain::DomainError;
use crate::domain::user::{User, UserRepository};

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn hash_password(password: String) -> Result<String, DomainError> {
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map(|h| h.to_string())
                .map_err(|e| DomainError::Unexpected(e.to_string()))
        })
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    #[instrument(skip(self, password), err, fields(db = "postgres"))]
    async fn create(&self, username: String, password: String) -> Result<i64, DomainError> {
        let hashed = Self::hash_password(password).await?;

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
        /*  Example of masking password, but we will return the hashed password for now
        Ok(row.map(|r| User {
            id: r.id,
            username: r.username,
            password: Self::mask_password(r.password),
            active: r.active,
        }))
        */
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
        // Example of masking password, but we will return the hashed password for now
        /*
        Ok(rows.into_iter().map(|r| User {
            id: r.id,
            username: r.username,
            password: Self::mask_password(r.password),
            active: r.active,
        }).collect())
        */
    }

    #[instrument(skip(self, password), err, fields(db = "postgres"))]
    async fn update(&self, id: i64, username: String, password: String) -> Result<User, DomainError> {
        let hashed = Self::hash_password(password).await?;

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
