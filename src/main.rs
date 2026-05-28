mod config;
mod models;
mod repository;
mod service;


use config::config::crear_pool;
use sqlx::pool;


#[tokio::main]
async fn main(){
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}");

    let pool = crear_pool()
        .await
        .expect("No se pudo conectar a la base de datos");

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}


fn unificar_routers(pool: sqlx::PgPool) -> axum::Router{

    let mut router1 = 
}
