-- Your SQL goes here
CREATE TABLE locatarios(
    idlocatario VARCHAR(64) PRIMARY KEY,
    idusuario VARCHAR(64) NOT NULL,
    idendereco VARCHAR(64) NOT NULL
)