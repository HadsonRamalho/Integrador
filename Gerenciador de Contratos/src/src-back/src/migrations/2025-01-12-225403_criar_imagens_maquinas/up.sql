-- Your SQL goes here
CREATE TABLE imagens_maquinas(
    idimagemmaquina VARCHAR(64) PRIMARY KEY,
    idimagem VARCHAR(64) NOT NULL,
    idmaquina VARCHAR(64) NOT NULL
)