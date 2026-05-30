use axum::{extract::{Path, State},http::StatusCode,Json,};

use sqlx::PgPool;
use crate::models::editoriales::{
    Editoriales,
    NuevaEditorial,
    ActualizarEditorial,
};
use crate::repository::editoriales_repository::EditorialesRepository;

pub async fn obtener_editoriales(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Editoriales>>, StatusCode> {
    let repo = EditorialesRepository::new(pool);

    match repo.obtener_editoriales().await {
        Ok(editoriales) => Ok(Json(editoriales)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn obtener_editorial_por_id(
    State(pool): State<PgPool>,
    Path(id_editorial): Path<i32>,
) -> Result<Json<Editoriales>, StatusCode> {
    let repo = EditorialesRepository::new(pool);

    match repo.obtener_editorial_por_id(id_editorial).await {
        Ok(Some(editorial)) => Ok(Json(editorial)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn crear_editorial(
    State(pool): State<PgPool>,
    Json(nueva_editorial): Json<NuevaEditorial>,
) -> Result<(StatusCode, Json<Editoriales>), StatusCode> {
    let repo = EditorialesRepository::new(pool);

    match repo.crear_editorial(nueva_editorial).await {
        Ok(editorial) => Ok((StatusCode::CREATED, Json(editorial))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn actualizar_editorial(
    State(pool): State<PgPool>,
    Path(id_editorial): Path<i32>,
    Json(editorial_actualizada): Json<ActualizarEditorial>,
) -> Result<Json<Editoriales>, StatusCode> {
    let repo = EditorialesRepository::new(pool);

    match repo
        .actualizar_editorial(id_editorial, editorial_actualizada)
        .await
    {
        Ok(editorial) => Ok(Json(editorial)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn actualizar_editorial_por_id(
    State(pool): State<PgPool>,
    Path(id_editorial): Path<i32>,
    Json(editorial_actualizada): Json<ActualizarEditorial>,
) -> Result<Json<Editoriales>, StatusCode> {
    let repo = EditorialesRepository::new(pool);

    match repo
        .actualizar_editorial(id_editorial, editorial_actualizada)
        .await
    {
        Ok(editorial) => Ok(Json(editorial)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn eliminar_editorial_por_id(
    State(pool): State<PgPool>,
    Path(id_editorial): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let repo = EditorialesRepository::new(pool);

    match repo.eliminar_editorial(id_editorial).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}