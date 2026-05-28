use axum::{extract::{Path, State},http::StatusCode,response::IntoResponse, Json};
use sqlx::PgPool;
use crate::models::autores::{NuevoAutor, ActualizarAutor};
use crate::repository::autores_repository::AutoresRepository;

pub async fn get_all_autores(State(pool): State<PgPool>) -> impl IntoResponse {
    let repo = AutoresRepository::new(pool);
    match repo.get_all().await {
        Ok(autores) => (StatusCode::OK, Json(autores)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}
    
pub async fn get_autor_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let repo = AutoresRepository::new(pool);
    match repo.get_by_id(id).await {
        Ok(Some(autor)) => (StatusCode::OK, Json(Some(autor))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn create_autor(State(pool): State<PgPool>, Json(nuevo_autor): Json<NuevoAutor>) -> impl IntoResponse {
    let repo = AutoresRepository::new(pool);
    match repo.create(nuevo_autor).await {
        Ok(Some(autor)) => (StatusCode::CREATED, Json(Some(autor))),
        Ok(None) => (StatusCode::BAD_REQUEST, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn update_autor(State(pool): State<PgPool>, Path(id): Path<i32>, Json(actualizar_autor): Json<ActualizarAutor>) -> impl IntoResponse {
    let repo = AutoresRepository::new(pool);
    match repo.update(id, actualizar_autor).await {
        Ok(Some(autor)) => (StatusCode::OK, Json(Some(autor))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn delete_autor(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let repo = AutoresRepository::new(pool);
    match repo.delete(id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}