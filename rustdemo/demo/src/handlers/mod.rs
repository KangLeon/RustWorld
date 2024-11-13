use crate::db::DbPool;
use crate::models::{CreateUser, User};
use actix_web::{web, Error, HttpResponse};

pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email)
        VALUES ($1, $2)
        RETURNING id, username, email, created_at
        "#,
        user.username,
        user.email
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = sqlx::query_as!(User, "SELECT id, username, email, created_at FROM users")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to get users: {:?}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().json(users))
}
