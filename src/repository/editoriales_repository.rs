use sqlx::PgPool;
use crate::models::editoriales::{
    Editoriales,
    NuevaEditorial,
    ActualizarEditorial,
};

pub struct EditorialesRepository {
    pool: PgPool,
}

impl EditorialesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_editoriales(&self) -> sqlx::Result<Vec<Editoriales>> {
        let editoriales = sqlx::query_as::<_, Editoriales>(
            "SELECT id_editorial, nombre, pais FROM editoriales"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(editoriales)
    }

    pub async fn obtener_editorial_por_id(&self, id_editorial: i32) -> sqlx::Result<Option<Editoriales>> {
        let editorial = sqlx::query_as::<_, Editoriales>(
            "SELECT id_editorial, nombre, pais
             FROM editoriales
             WHERE id_editorial = $1"
        )
        .bind(id_editorial)
        .fetch_optional(&self.pool)
        .await?;

        Ok(editorial)
    }

    pub async fn crear_editorial(&self, nueva_editorial: NuevaEditorial) -> sqlx::Result<Editoriales> {
        let editorial = sqlx::query_as::<_, Editoriales>(
            "INSERT INTO editoriales (nombre, pais)
             VALUES ($1, $2)
             RETURNING id_editorial, nombre, pais"
        )
        .bind(nueva_editorial.nombre)
        .bind(nueva_editorial.pais)
        .fetch_one(&self.pool)
        .await?;

        Ok(editorial)
    }

    pub async fn actualizar_editorial(&self, id_editorial: i32, editorial_actualizada: ActualizarEditorial) -> sqlx::Result<Editoriales> {
        let editorial = sqlx::query_as::<_, Editoriales>(
            "UPDATE editoriales
             SET nombre = COALESCE($1, nombre),
                 pais = COALESCE($2, pais)
             WHERE id_editorial = $3
             RETURNING id_editorial, nombre, pais"
        )
        .bind(editorial_actualizada.nombre)
        .bind(editorial_actualizada.pais)
        .bind(id_editorial)
        .fetch_one(&self.pool)
        .await?;

        Ok(editorial)
    }

    pub async fn eliminar_editorial(&self, id_editorial: i32) -> sqlx::Result<()> {
        sqlx::query(
            "DELETE FROM editoriales WHERE id_editorial = $1"
        )
        .bind(id_editorial)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}