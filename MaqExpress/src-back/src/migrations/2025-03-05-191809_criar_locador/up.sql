-- Your SQL goes here
CREATE TABLE locadoras(
    idlocadora VARCHAR(64) PRIMARY KEY,
    idusuario VARCHAR(64) NOT NULL,
    idendereco VARCHAR(64) NOT NULL,
    idconta VARCHAR(64) NOT NULL
);