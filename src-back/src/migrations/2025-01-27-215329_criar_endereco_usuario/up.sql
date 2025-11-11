-- Your SQL goes here
CREATE TABLE enderecos_usuarios(
    idenderecousuario VARCHAR(64) PRIMARY KEY,
    idendereco VARCHAR(64) NOT NULL,
    idusuario VARCHAR(64) NOT NULL
)