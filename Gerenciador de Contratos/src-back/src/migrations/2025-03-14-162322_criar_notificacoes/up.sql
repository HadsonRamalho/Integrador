-- Your SQL goes here
CREATE TABLE notificacoes(
  idnotificacao VARCHAR(64) PRIMARY KEY,
  idusuario VARCHAR(64) NOT NULL,
  titulo VARCHAR(128) NOT NULL,
  mensagem TEXT NOT NULL,
  onclick VARCHAR(64) NOT NULL,
  status VARCHAR(64) NOT NULL,
  datacriacao TIMESTAMP NOT NULL
);