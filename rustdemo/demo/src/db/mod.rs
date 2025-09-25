use sea_orm::{Database, DatabaseConnection, DbErr};

pub type DbPool = DatabaseConnection;

pub async fn init_db(database_url: &str) -> Result<DbPool, DbErr> {
    Database::connect(database_url).await
}
