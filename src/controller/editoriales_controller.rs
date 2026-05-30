use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use crate::service::editoriales_service::{
    obtener_editoriales,
    obtener_editorial_por_id,
    crear_editorial,
    actualizar_editorial_por_id,
    eliminar_editorial_por_id,
};

pub fn editoriales_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(obtener_editoriales))
        .route("/", post(crear_editorial))
        .route("/{id_editorial}", get(obtener_editorial_por_id))
        .route("/{id_editorial}", put(actualizar_editorial_por_id))
        .route("/{id_editorial}", delete(eliminar_editorial_por_id))
        .with_state(pool)
}