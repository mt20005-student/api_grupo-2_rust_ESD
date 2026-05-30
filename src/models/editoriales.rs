use serde::{Deserialize,Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Editoriales {
    pub id_editorial: i32,
    pub nombre: String,
    pub pais: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaEditorial {
    pub nombre: String,
    pub pais: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarEditorial {
    pub nombre: Option<String>,
    pub pais: Option<String>,
}