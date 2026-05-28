use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub fn obtener_url_base_datos() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE NO ESTA CONFIGURADO EN EL .env")
}

pub async fn crear_pool() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>>{
    let url_base_datos = obtener_url_base_datos();

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url_base_datos)
        .await
}
