use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// SeaORM 实体定义 - 非常优雅！
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(unique)]
    pub username: String,

    #[sea_orm(unique)]
    pub email: String,

    pub created_at: NaiveDateTime,
}

// SeaORM 关系定义（如果需要）
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// 类型别名，让代码更清晰
pub type User = Model;
pub type UserEntity = Entity;

// 创建用户的请求模型
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}
