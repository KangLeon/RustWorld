mod config;
mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载配置
    let config = config::Config::from_env().expect("Failed to load config");

    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // 初始化数据库
    let pool = db::init_db(&config.database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!(
        "Starting server at {}:{}",
        config.server_host,
        config.server_port
    );

    // 启动服务器
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/api")
                .route("/users", web::post().to(handlers::create_user))
                .route("/users", web::get().to(handlers::get_users))
                .route("/users/{id}", web::get().to(handlers::get_user_by_id)),
        )
    })
    .bind(format!("{}:{}", config.server_host, config.server_port))?
    .run()
    .await
}
