CREATE TABLE autores (
    id_autor SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    nacionalidad VARCHAR(50)
);
CREATE TABLE editoriales (
    id_editorial SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    pais VARCHAR(50)
);
CREATE TABLE usuarios (
    id_usuario SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    direccion TEXT,
    fecha_registro DATE DEFAULT CURRENT_DATE
);
CREATE TABLE libros (
    id_libro SERIAL PRIMARY KEY,
    titulo VARCHAR(150) NOT NULL,
    isbn VARCHAR(20) UNIQUE,
    anio_publicacion INT,
    id_autor INT REFERENCES autores(id_autor),
    id_editorial INT REFERENCES editoriales(id_editorial)
);
CREATE TABLE prestamos (
    id_prestamo SERIAL PRIMARY KEY,
    id_libro INT REFERENCES libros(id_libro),
    id_usuario INT REFERENCES usuarios(id_usuario),
    fecha_salida DATE DEFAULT CURRENT_DATE,
    fecha_devolucion DATE
);