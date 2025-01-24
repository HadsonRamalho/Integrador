-- Your SQL goes here
CREATE TABLE sessoes(
    idsessao VARCHAR(64) PRIMARY KEY,
    idusuario VARCHAR(64) NOT NULL,
    identificador VARCHAR(64) NOT NULL,
    expiracao TIMESTAMP WITH TIME ZONE NOT NULL,
)