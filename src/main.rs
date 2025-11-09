use dotenvy::dotenv;
use migration::MigratorTrait;
use sea_orm::Database;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let _generate_entities = migration::Migrator::up(&db, None).await;

    println!("✅ Banco de dados atualizado com sucesso!");
}
