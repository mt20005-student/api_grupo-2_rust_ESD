use sqlx::{PgPool, Row};
use crate::models::libros::{Libros, NuevoLibro, ActualizarLibro};

pub struct LibrosRepository {
    pool: PgPool,
}

impl LibrosRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_libros(&self) -> sqlx::Result<Vec<Libros>>{
        let filas = sqlx::query("SELECT id_libro, titulo, isbn, anio_publicacion, id_autor, id_editorial FROM Libros")
            .fetch_all(&self.pool)
            .await?;

        let libros = filas.into_iter().map(|fila| {
            Libros {
                id_libro: fila.get("id_libro"),
                titulo: fila.get("titulo"),
                isbn: fila.get("isbn"),
                anio_publicacion: fila.get("anio_publicacion"),
                id_autor: fila.get("id_autor"),
                id_editorial: fila.get("id_editorial"),
            }
        }).collect();
        Ok(libros)
    }

    pub async fn crear_libro(&self, nuevo_libro: NuevoLibro) -> sqlx::Result<NuevoLibro>{
        let fila = sqlx::query("INSERT INTO libros (titulo, isbn, anio_publicacion, id_autor, id_editorial) VALUES ($1, $2, $3, $4, $5) RETURNING id_libro, titulo, isbn, anio_publicacion, id_autor, id_editorial" )
            .bind(nuevo_libro.titulo)
            .bind(nuevo_libro.isbn)
            .bind(nuevo_libro.anio_publicacion)
            .bind(nuevo_libro.id_autor)
            .bind(nuevo_libro.id_editorial)
            .fetch_one(&self.pool)
            .await?;

        Ok(NuevoLibro {
            titulo: fila.get("titulo"),
            isbn: fila.get("isbn"),
            anio_publicacion: fila.get("anio_publicacion"),
            id_autor: fila.get("id_autor"),
            id_editorial: fila.get("id_editorial")
        })
    }

    pub async fn actualizar_libro(&self, id_libro: i32, libro_actualizado: ActualizarLibro) -> sqlx::Result<Libros> {
        let fila = sqlx::query("UPDATE libros SET titulo = $1, isbn = $2, anio_publicacion = $3, id_autor = $4, id_editorial = $5 WHERE id_libro = $6 RETURNING id_libro, titulo, isbn, anio_publicacion, id_autor, id_editorial")
            .bind(libro_actualizado.titulo)
            .bind(libro_actualizado.isbn)
            .bind(libro_actualizado.anio_publicacion)
            .bind(libro_actualizado.id_autor)
            .bind(libro_actualizado.id_editorial)
            .bind(id_libro)
            .fetch_one(&self.pool)
            .await?;

        Ok(Libros {
            id_libro: fila.get("id_libro"),
            titulo: fila.get("titulo"),
            isbn: fila.get("isbn"),
            anio_publicacion: fila.get("anio_publicacion"),
            id_autor: fila.get("id_autor"),
            id_editorial: fila.get("id_editorial")
        })
    }

    pub async fn eliminar_libro(&self, id_libro: i32) -> sqlx::Result<()>{
        sqlx::query("DELETE FROM libros WHERE id_libro = $1")
            .bind(id_libro)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn obtener_libro_por_id(&self, id_libro: i32) -> sqlx::Result<Option<Libros>> {
        let fila = sqlx::query("SELECT id_libro, titulo, isbn, anio_publicacion, id_autor, id_editorial FROM Libros WHERE id_libro = $1")
            .bind(id_libro)
            .fetch_optional(&self.pool) 
            .await?;

        let libro = fila.map(|fila| {
            Libros {
                id_libro: fila.get("id_libro"),
                titulo: fila.get("titulo"),
                isbn: fila.get("isbn"),
                anio_publicacion: fila.get("anio_publicacion"),
                id_autor: fila.get("id_autor"),
                id_editorial: fila.get("id_editorial"),
            }
        });

        Ok(libro)
    }

}