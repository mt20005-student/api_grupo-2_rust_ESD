use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;
use crate::models::usuarios::{Usuario, NuevoUsuario, ActualizarUsuario};
use crate::repository::usuarios_repository::UsuariosRepository;

pub async fn obtener_todos(State(pool): State<PgPool>) -> impl IntoResponse {
    let repo = UsuariosRepository::new(pool);
    match repo.obtener_todos().await {
        Ok(usuarios) => (StatusCode::OK, Json(usuarios)),
        // Importante: Si la de arriba devuelve Json, esta también (aunque sea un vector vacío)
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

pub async fn obtener_por_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let repo = UsuariosRepository::new(pool);
    match repo.obtener_por_id(id).await {
        Ok(Some(u)) => (StatusCode::OK, Json(Some(u))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn crear(State(pool): State<PgPool>, Json(payload): Json<NuevoUsuario>) -> impl IntoResponse {
    let repo = UsuariosRepository::new(pool);
    match repo.crear(payload).await {
        // Asumiendo que tu repo devuelve el Usuario creado
        Ok(u) => (StatusCode::CREATED, Json(Some(u))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn actualizar(State(pool): State<PgPool>, Path(id): Path<i32>, Json(payload): Json<ActualizarUsuario>) -> impl IntoResponse {
    let repo = UsuariosRepository::new(pool);
    match repo.actualizar(id, payload).await {
        Ok(u) => (StatusCode::OK, Json(Some(u))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn eliminar(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    let repo = UsuariosRepository::new(pool);
    match repo.eliminar(id).await {
        // Si se eliminó al menos una fila (u64 > 0)
        Ok(filas) if filas > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}