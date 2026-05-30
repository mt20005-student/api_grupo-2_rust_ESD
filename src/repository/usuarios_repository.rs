use sqlx::{PgPool, Row};
use crate::models::usuarios::{Usuario, NuevoUsuario, ActualizarUsuario};

pub struct UsuariosRepository {
    pool: PgPool,
}

impl UsuariosRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    //  Obtener todos los usuarios
    pub async fn obtener_todos(&self) -> Result<Vec<Usuario>, sqlx::Error> {
        let filas = sqlx::query("SELECT id_usuario, nombre, direccion, fecha_registro FROM Usuarios")
            .fetch_all(&self.pool)
            .await?;

        let usuarios = filas.into_iter().map(|fila| {
            Usuario {
                id_usuario: fila.get("id_usuario"),
                nombre: fila.get("nombre"),
                direccion: fila.get("direccion"),
                fecha_registro: fila.get("fecha_registro"),
            }
        }).collect();

        Ok(usuarios)
    }

    // Obtener por ID
    pub async fn obtener_por_id(&self, id: i32) -> Result<Option<Usuario>, sqlx::Error> {
        let fila = sqlx::query("SELECT id_usuario, nombre, direccion, fecha_registro FROM Usuarios WHERE id_usuario = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let usuario = fila.map(|f| Usuario {
            id_usuario: f.get("id_usuario"),
            nombre: f.get("nombre"),
            direccion: f.get("direccion"),
            fecha_registro: f.get("fecha_registro"),
        });

        Ok(usuario)
    }

    //  Crear usuario (Usamos RETURNING para obtener la fila creada)
    pub async fn crear(&self, datos: NuevoUsuario) -> Result<Usuario, sqlx::Error> {
        let fila = sqlx::query("INSERT INTO Usuarios (nombre, direccion) VALUES ($1, $2) RETURNING id_usuario, nombre, direccion, fecha_registro")
            .bind(&datos.nombre)
            .bind(&datos.direccion)
            .fetch_one(&self.pool)
            .await?;

        Ok(Usuario {
            id_usuario: fila.get("id_usuario"),
            nombre: fila.get("nombre"),
            direccion: fila.get("direccion"),
            fecha_registro: fila.get("fecha_registro"),
        })
    }

    // Actualizar usuario
    pub async fn actualizar(&self, id: i32, datos: ActualizarUsuario) -> Result<Usuario, sqlx::Error> {
        let fila = sqlx::query(
            "UPDATE Usuarios SET nombre = COALESCE($1, nombre), direccion = COALESCE($2, direccion) 
             WHERE id_usuario = $3 RETURNING id_usuario, nombre, direccion, fecha_registro"
        )
        .bind(&datos.nombre)
        .bind(&datos.direccion)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Usuario {
            id_usuario: fila.get("id_usuario"),
            nombre: fila.get("nombre"),
            direccion: fila.get("direccion"),
            fecha_registro: fila.get("fecha_registro"),
        })
    }

    //  Eliminar usuario
    pub async fn eliminar(&self, id: i32) -> Result<u64, sqlx::Error> {
        let resultado = sqlx::query("DELETE FROM Usuarios WHERE id_usuario = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(resultado.rows_affected())
    }
}