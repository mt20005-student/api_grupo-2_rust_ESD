use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Autores {
    pub id_autor: i32,
    pub nombre: String,
    pub nacionalidad: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoAutor {
    pub nombre: String,
    pub nacionalidad: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarAutor {
    pub nombre: Option<String>,
    pub nacionalidad: Option<Option<String>>,
}