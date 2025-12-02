use crate::{
    domain::repositories::user_repository::UserRepository,
    infrastructure::repositories::postgres::user_repository_sqlx::UserRepositorySQLx,
};
use sqlx::PgPool;
use std::sync::Arc;

pub struct RepositoryBundle {
    pub user_repo: Arc<dyn UserRepository>,
}

impl RepositoryBundle {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_repo: Arc::new(UserRepositorySQLx::new(pool.clone())),
        }
    }
}
