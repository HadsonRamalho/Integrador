-- Your SQL goes here
CREATE TABLE contratos(
    idcontrato VARCHAR(64) PRIMARY KEY,
    idlocatario VARCHAR(64) NOT NULL,
    idlocador VARCHAR(64) NOT NULL,
    idenderecolocatario VARCHAR(64) NOT NULL,
    idenderecolocador VARCHAR(64) NOT NULL,
    idenderecoretirada VARCHAR(64) NOT NULL,
    idmaquina VARCHAR(64) NOT NULL,
    idsolicitacaocontrato VARCHAR(64) NOT NULL,
    idcontabancarialocador VARCHAR(64) NOT NULL,
    medidatempolocacao VARCHAR(64) NOT NULL,
    cidadeforo VARCHAR(64) NOT NULL,
    statuscontrato VARCHAR(64) NOT NULL,
    prazolocacao FLOAT NOT NULL,
    valorlocacao FLOAT NOT NULL,
    datacontrato TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);