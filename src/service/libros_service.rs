use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::libros::{Libros, NuevoLibro, ActualizarLibro};
use crate::repository::libros_repository::LibrosRepository;


pub async fn obtener_libros(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Libros>>, StatusCode> {
    let repo = LibrosRepository::new(pool);
    
    match repo.obtener_libros().await {
        Ok(libros) => Ok(Json(libros)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR), // 500 Internal Server Error
    }
}


pub async fn obtener_libro_por_id(
    State(pool): State<PgPool>,
    Path(id_libro): Path<i32>,
) -> Result<Json<Libros>, StatusCode> {
    let repo = LibrosRepository::new(pool);
    
    match repo.obtener_libro_por_id(id_libro).await {
        Ok(Some(libro)) => Ok(Json(libro)),
        Ok(None) => Err(StatusCode::NOT_FOUND), // 404 si el libro no existe
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


pub async fn crear(
    State(pool): State<PgPool>,
    Json(nuevo_libro): Json<NuevoLibro>,
) -> Result<(StatusCode, Json<NuevoLibro>), StatusCode> {
    let repo = LibrosRepository::new(pool);
    
    match repo.crear_libro(nuevo_libro).await {
        Ok(libro) => Ok((StatusCode::CREATED, Json(libro))), 
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


pub async fn actualizar_libro(
    State(pool): State<PgPool>,
    Path(id_libro): Path<i32>,
    Json(libro_actualizado): Json<ActualizarLibro>,
) -> Result<Json<Libros>, StatusCode> {
    let repo = LibrosRepository::new(pool);

    match repo.actualizar_libro(id_libro, libro_actualizado).await {
        Ok(libro) => Ok(Json(libro)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),       
    }
}

pub async fn eliminar_libro_por_id(
    State(pool): State<PgPool>, 
    Path(id_libro): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let repo = LibrosRepository::new(pool);
    
    match repo.eliminar_libro(id_libro).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT), 
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),            
    }
}