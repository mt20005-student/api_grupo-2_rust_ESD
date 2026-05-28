use sqlx::{PgPool, Row};
use crate::models::autores::{Autores, NuevoAutor, ActualizarAutor};

pub struct AutoresRepository {
    pool: PgPool,
}

impl AutoresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Autores>, sqlx::Error> {
        let autores = sqlx::query_as::<_, Autores>("SELECT id_autor, nombre, nacionalidad FROM autores")
            .fetch_all(&self.pool)
            .await?;
        Ok(autores)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Autores>, sqlx::Error> {
        let autor = sqlx::query_as::<_, Autores>("SELECT id_autor, nombre, nacionalidad FROM autores WHERE id_autor = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(autor)
    }

    pub async fn create(&self, nuevo_autor: NuevoAutor) -> Result<Option<Autores>, sqlx::Error> {
        let autor = sqlx::query_as::<_, Autores>(
            "INSERT INTO autores (nombre, nacionalidad) VALUES ($1, $2) RETURNING id_autor, nombre, nacionalidad"
        ).bind(nuevo_autor.nombre)
            .bind(nuevo_autor.nacionalidad)
            .fetch_optional(&self.pool)
            .await?;
        Ok(autor)
    }

    pub async fn update(&self, id: i32, actualizar_autor: ActualizarAutor) -> Result<Option<Autores>, sqlx::Error> {
        let autor = sqlx::query_as::<_, Autores>(
            "UPDATE autores SET nombre = COALESCE($1, nombre), nacionalidad = COALESCE($2, nacionalidad) WHERE id_autor = $3 RETURNING id_autor, nombre, nacionalidad"
        ).bind(actualizar_autor.nombre)
            .bind(actualizar_autor.nacionalidad)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(autor)
    }

    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM autores WHERE id_autor = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}