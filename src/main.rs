use service::ports::input::user::UserRepository;
use service::adaptateur::{
    output::user::PostgresUserRepository,
};
use  service::adaptateur::input::user::configure;


use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::PgPool;
use std::sync::Arc;
use  std::env;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    let user_repository = Arc::new(PostgresUserRepository::new(pool)) as Arc<dyn UserRepository>;
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(user_repository.clone()))
            .configure(configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}