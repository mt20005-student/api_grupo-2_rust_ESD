use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
// Importamos las funciones desde el service
use crate::service::usuarios_service::{obtener_todos, obtener_por_id, crear, actualizar, eliminar};

pub fn usuarios_router(pool: PgPool) -> Router {
    Router::new()
        // Rutas base
        .route("/", get(obtener_todos))
        .route("/", post(crear))
        // Rutas con ID (Usando llaves {} por la versión de Axum)
        .route("/{id}", get(obtener_por_id))
        .route("/{id}", put(actualizar))
        .route("/{id}", delete(eliminar))
        .with_state(pool)
}