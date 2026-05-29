use serde::{Serialize, Deserialize};
use sqlx::FromRow;

// Usamos chrono para manejar la fecha de registro de la DB
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Usuario{
    pub id_usuario: i32, // i32 representa a tipo SERIAL en postgres
    pub nombre:String,
    pub direccion: Option<String>, // Option porque puede ser NULL
    pub fecha_registro: Option<NaiveDate>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct NuevoUsuario {
    pub nombre: String,
    pub direccion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActualizarUsuario {
    pub nombre: Option<String>,
    pub direccion: Option<String>,
}