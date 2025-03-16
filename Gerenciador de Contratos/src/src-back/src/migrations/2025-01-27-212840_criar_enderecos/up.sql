-- Your SQL goes here
CREATE TABLE enderecos(
    idendereco VARCHAR(64) PRIMARY KEY,
    pais VARCHAR(64) NOT NULL,
    estado VARCHAR(64) NOT NULL,
    cidade VARCHAR(64) NOT NULL,
    cep VARCHAR(12) NOT NULL,
    bairro VARCHAR(64) NOT NULL,
    logradouro VARCHAR(64) NOT NULL,
    numero VARCHAR(64) NOT NULL,
    complemento VARCHAR(64) NOT NULL
);