-- Your SQL goes here
CREATE TABLE usuarios(
    nome VARCHAR(64) NOT NULL,
    email VARCHAR(64) NOT NULL,
    senha VARCHAR(64) NOT NULL,
    documento VARCHAR(64) NOT NULL,
    datacadastro TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    idusuario VARCHAR(64) PRIMARY KEY
);