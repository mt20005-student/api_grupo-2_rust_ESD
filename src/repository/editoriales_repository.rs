use sqlx::{PgPool, Row};
use crate::models::editoriales::{Editoriales, NuevaEditorial, ActualizarEditorial};

pub struct EditorialesRepository {
    pool: PgPool,
}

impl EditorialesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_editoriales(&self) -> Result<Vec<Editoriales>, sqlx::Error> {
        let editoriales = sqlx::query_as::<_, Editoriales>(
            "SELECT id_editorial, nombre, pais FROM editoriales"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(editoriales)
    }

    pub async fn obtener_editorial_por_id(&self, id: i32) -> Result<Option<Editoriales>, sqlx::Error> {
        let editorial = sqlx::query_as::<_, Editoriales>(
            "SELECT id_editorial, nombre, pais FROM editoriales WHERE id_editorial = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(editorial)
    }

    pub async fn crear_editorial(&self, nueva_editorial: NuevaEditorial) -> Result<Option<Editoriales>, sqlx::Error> {
        let editorial = sqlx::query_as::<_, Editoriales>(
            "INSERT INTO editoriales (nombre, pais)
             VALUES ($1, $2)
             RETURNING id_editorial, nombre, pais"
        )
        .bind(nueva_editorial.nombre)
        .bind(nueva_editorial.pais)
        .fetch_optional(&self.pool)
        .await?;

        Ok(editorial)
    }

    pub async fn actualizar_editorial(
        &self,
        id: i32,
        actualizar_editorial: ActualizarEditorial,
    ) -> Result<Option<Editoriales>, sqlx::Error> {
        let editorial = sqlx::query_as::<_, Editoriales>(
            "UPDATE editoriales
             SET nombre = COALESCE($1, nombre),
                 pais = COALESCE($2, pais)
             WHERE id_editorial = $3
             RETURNING id_editorial, nombre, pais"
        )
        .bind(actualizar_editorial.nombre)
        .bind(actualizar_editorial.pais)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(editorial)
    }

    pub async fn eliminar_editorial(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM editoriales WHERE id_editorial = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}