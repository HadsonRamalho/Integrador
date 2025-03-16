-- Your SQL goes here
CREATE TABLE maquinas_usuarios(
    idmaquinausuario VARCHAR(64) PRIMARY KEY,
    idmaquina VARCHAR(64) NOT NULL,
    idusuario VARCHAR(64) NOT NULL
)