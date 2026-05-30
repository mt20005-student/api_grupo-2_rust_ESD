use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::editoriales::{NuevaEditorial, ActualizarEditorial};
use crate::repository::editoriales_repository::EditorialesRepository;

pub async fn obtener_editoriales(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let repo = EditorialesRepository::new(pool);
    match repo.obtener_editoriales().await {
        Ok(editoriales) => (StatusCode::OK, Json(editoriales)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

pub async fn obtener_editorial_por_id(
    State(pool): State<PgPool>,
    Path(id_editorial): Path<i32>,
) -> impl IntoResponse {
    let repo = EditorialesRepository::new(pool);
    match repo.obtener_editorial_por_id(id_editorial).await {
        Ok(Some(editorial)) => (StatusCode::OK, Json(Some(editorial))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn crear_editorial(
    State(pool): State<PgPool>,
    Json(nueva_editorial): Json<NuevaEditorial>,
) -> impl IntoResponse {
    let repo = EditorialesRepository::new(pool);
    match repo.crear_editorial(nueva_editorial).await {
        Ok(editorial) => (StatusCode::CREATED, Json(Some(editorial))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn actualizar_editorial_por_id(
    State(pool): State<PgPool>,
    Path(id_editorial): Path<i32>,
    Json(editorial_actualizada): Json<ActualizarEditorial>,
) -> impl IntoResponse {
    let repo = EditorialesRepository::new(pool);
    match repo.actualizar_editorial(id_editorial, editorial_actualizada).await {
        Ok(editorial) => (StatusCode::OK, Json(Some(editorial))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn eliminar_editorial_por_id(
    State(pool): State<PgPool>,
    Path(id_editorial): Path<i32>,
) -> impl IntoResponse {
    let repo = EditorialesRepository::new(pool);
    match repo.eliminar_editorial(id_editorial).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}