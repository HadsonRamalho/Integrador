-- Your SQL goes here
CREATE TABLE solicitacoes_contratos(
    idsolicitacao VARCHAR(64) PRIMARY KEY,
    idlocador VARCHAR(64) NOT NULL,
    idlocatario VARCHAR(64) NOT NULL,
    idmaquina VARCHAR(64) NOT NULL,
    medidatempolocacao VARCHAR(64) NOT NULL,
    origemsolicitacao VARCHAR(64) NOT NULL,
    statussolicitacao VARCHAR(64) NOT NULL,
    prazolocacao FLOAT NOT NULL,
    valorsolicitacao FLOAT NOT NULL,
    datasolicitacao TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);