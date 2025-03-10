-- Your SQL goes here
CREATE TABLE contratos(
    idcontrato VARCHAR(64) PRIMARY KEY,
    idlocatario VARCHAR(64) NOT NULL,
    idlocador VARCHAR(64) NOT NULL,
    idenderecolocatario VARCHAR(64) NOT NULL,
    idenderecolocador VARCHAR(64) NOT NULL,
    idenderecoretirada VARCHAR(64) NOT NULL,
    idmaquina VARCHAR(64) NOT NULL,
    prazolocacao VARCHAR(64) NOT NULL,
    medidatempolocacao VARCHAR(64) NOT NULL,
    valorlocacao VARCHAR(64) NOT NULL,
    contabancarialocador VARCHAR(64) NOT NULL,
    cidadeforo VARCHAR(64) NOT NULL,
    datacontrato TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    statuscontrato VARCHAR(64) NOT NULL
);