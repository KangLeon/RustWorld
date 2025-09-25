use crate::db::DbPool;
use crate::models::{CreateUser, UserEntity};
use actix_web::{web, Error, HttpResponse};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

// 🎉 看这个优雅的语法！
pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    // SeaORM 的 ActiveModel 模式 - 超级干净！
    let new_user = crate::models::ActiveModel {
        username: Set(user.username.clone()),
        email: Set(user.email.clone()),
        ..Default::default()
    };

    let user = new_user.insert(pool.get_ref()).await.map_err(|e| {
        tracing::error!("Failed to create user: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(user))
}

// 🚀 这个更简单！
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = UserEntity::find().all(pool.get_ref()).await.map_err(|e| {
        tracing::error!("Failed to get users: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(users))
}

// 🔥 额外福利：根据ID查找用户 - 一行代码！
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let user = UserEntity::find_by_id(user_id)
        .one(pool.get_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to find user: {:?}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::NotFound().json("User not found")),
    }
}
