-- Your SQL goes here
CREATE TABLE imagens(
    idimagem VARCHAR(64) PRIMARY KEY,
    nome VARCHAR(128) NOT NULL,
    bin BYTEA NOT NULL,
    link VARCHAR(128) NOT NULL
)