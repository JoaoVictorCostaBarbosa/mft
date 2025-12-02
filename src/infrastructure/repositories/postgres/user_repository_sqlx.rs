use crate::{
    domain::{
        entities::user::User,
        errors::{domain_error::DomainError, user_error::UserError},
        repositories::user_repository::UserRepository,
    },
    infrastructure::repositories::{enums_db::role_db::RoleDb, models::user_model::UserModel},
};
use axum::async_trait;
use chrono::{Local, NaiveDateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepositorySQLx {
    pool: PgPool,
}

impl UserRepositorySQLx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositorySQLx {
    async fn create_user(&self, user: &User) -> Result<(), DomainError> {
        let role: RoleDb = user.role.clone().into();

        sqlx::query(
            r#"
            INSERT INTO users
            (id, name, email, password, role, url_img, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(user.id)
        .bind(user.name.value())
        .bind(user.email.value())
        .bind(user.password.clone())
        .bind(role)
        .bind(user.url_img.clone())
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(user.deleted_at)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, DomainError> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT * FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(DomainError::from)?;

        let user = result.to_domain()?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT * FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(user_model) => {
                let user = user_model.to_domain()?;
                Ok(user)
            }
            Err(e) => Err(DomainError::from(e)),
        }
    }

    async fn get_all_users(&self) -> Result<Vec<User>, DomainError> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT * FROM users
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(DomainError::from)?;

        let users = result
            .iter()
            .map(|user| user.to_domain())
            .collect::<Result<Vec<_>, UserError>>()?;

        Ok(users)
    }

    async fn update_user_name(&self, name: String, user_id: Uuid) -> Result<(), DomainError> {
        let now: NaiveDateTime = Local::now().naive_local();

        sqlx::query(
            r#"
            UPDATE users
            SET name = $1,
                updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(name)
        .bind(now)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn update_user_email(&self, email: String, user_id: Uuid) -> Result<(), DomainError> {
        let now: NaiveDateTime = Local::now().naive_local();

        sqlx::query(
            r#"
            UPDATE users
            SET email = $1,
                updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(email)
        .bind(now)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn update_user_password(
        &self,
        password: String,
        user_id: Uuid,
    ) -> Result<(), DomainError> {
        let now: NaiveDateTime = Local::now().naive_local();

        sqlx::query(
            r#"
            UPDATE users
            SET password = $1,
                updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(password)
        .bind(now)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn update_user_avatar(
        &self,
        avatar_url: String,
        user_id: Uuid,
    ) -> Result<(), DomainError> {
        let now: NaiveDateTime = Local::now().naive_local();

        sqlx::query(
            r#"
            UPDATE users
            SET url_img = $1,
                updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(avatar_url)
        .bind(now)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn soft_delete_user(&self, user_id: Uuid) -> Result<(), DomainError> {
        let now: NaiveDateTime = Local::now().naive_local();

        sqlx::query(
            r#"
            UPDATE users
            SET deleted_at = $1,
                updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(now)
        .bind(now)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }
}
