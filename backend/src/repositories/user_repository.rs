use bcrypt::{hash, DEFAULT_COST};
use crate::db::DbPool;
use crate::domain::user::{User, CreateUser};
use std::env;

pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, mut user: CreateUser) -> Result<i64, sqlx::Error> {
        // Hash password if present
        if let Some(ref password) = user.password {
            let hashed = hash(password, DEFAULT_COST).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
            user.password = Some(hashed);
        }

        // Determine role
        let admin_email = env::var("ADMIN_EMAIL").unwrap_or_default();
        let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;
        
        let role = if user_count.0 == 0 || user.email == admin_email {
            "admin".to_string()
        } else {
            user.role.unwrap_or_else(|| "user".to_string())
        };

        let result = sqlx::query(
            "INSERT INTO users (name, email, password, contact, role) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.contact)
        .bind(role)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, contact, google_id, avatar_url, role, created_at FROM users"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, contact, google_id, avatar_url, role, created_at FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, contact, google_id, avatar_url, role, created_at FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update(&self, id: i32, user: CreateUser) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE users SET name = ?, email = ?, contact = ?, role = ? WHERE id = ?"
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.contact)
        .bind(&user.role.unwrap_or_else(|| "user".to_string()))
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn delete(&self, id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM users WHERE id = ?"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn find_or_create_google_user(&self, google_user: crate::domain::oauth::GoogleUser) -> Result<User, sqlx::Error> {
        let existing = self.get_by_email(&google_user.email).await?;
        if let Some(user) = existing {
            // Update google_id and avatar if missing
            sqlx::query("UPDATE users SET google_id = ?, avatar_url = ? WHERE id = ?")
                .bind(&google_user.id)
                .bind(&google_user.picture)
                .bind(user.id)
                .execute(&self.pool)
                .await?;
            return Ok(user);
        }

        // Create new
        let admin_email = env::var("ADMIN_EMAIL").unwrap_or_default();
        let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;
        
        let role = if user_count.0 == 0 || google_user.email == admin_email {
            "admin".to_string()
        } else {
            "user".to_string()
        };

        let result = sqlx::query(
            "INSERT INTO users (name, email, google_id, avatar_url, role) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&google_user.name)
        .bind(&google_user.email)
        .bind(&google_user.id)
        .bind(&google_user.picture)
        .bind(role)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid() as i32;
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }
}
