use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use crate::service::autores_service::{get_all_autores, get_autor_by_id, create_autor, update_autor, delete_autor};


pub fn autores_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(get_all_autores))
        .route("/", post(create_autor))
        .route("/{id}", get(get_autor_by_id))
        .route("/{id}", put(update_autor))
        .route("/{id}", delete(delete_autor))
        .with_state(pool)
}