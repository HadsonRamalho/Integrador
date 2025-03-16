-- Your SQL goes here
CREATE TABLE maquinas(
    idmaquina VARCHAR(64) PRIMARY KEY,
    idpublico VARCHAR(64) NOT NULL,
    nome VARCHAR(64) NOT NULL,
    numeroserie VARCHAR(64) NOT NULL,
    categoria VARCHAR(64) NOT NULL,
    valoraluguel FLOAT NOT NULL,
    disponivelaluguel VARCHAR(64) NOT NULL,
    status VARCHAR(64) NOT NULL,
    datacadastro TIMESTAMP NOT NULL,
    dataatualizacao TIMESTAMP NOT NULL,
    descricao VARCHAR(64) NOT NULL    
);