mod config;
mod models;
mod repository;
mod service;
mod controller;


use config::config::crear_pool;
use controller::autores_controller::autores_router;


use controller::usuarios_controller::usuarios_router;



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


    fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
        // Aquí puedes agregar más routers para otras entidades como libros, etc.
        let api_routes = axum::Router::new()
            .nest("/autores", autores_router(pool.clone()))
            .nest("/usuarios", usuarios_router(pool.clone()));

        axum::Router::new().nest("/api/v1", api_routes)
    }