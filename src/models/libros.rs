use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Libros{
    pub id_libro: i32,
    pub titulo: String,
    pub isbn: String,
    pub anio_publicacion: i32,
    pub id_autor: i32,
    pub id_editorial: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoLibro{
    pub titulo: String,
    pub isbn: String,
    pub anio_publicacion: i32,
    pub id_autor: i32,
    pub id_editorial: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarLibro{
    pub titulo: String,
    pub isbn: String,
    pub anio_publicacion: i32,
    pub id_autor: i32,
    pub id_editorial: i32,
}