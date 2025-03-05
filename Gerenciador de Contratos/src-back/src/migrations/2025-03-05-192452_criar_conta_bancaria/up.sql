-- Your SQL goes here
CREATE TABLE contas_bancarias(
    idconta VARCHAR(64) PRIMARY KEY,
    idusuario VARCHAR(64) NOT NULL,
    numeroconta VARCHAR(64) NOT NULL,
    numeroagencia VARCHAR(64) NOT NULL,
    nomebanco VARCHAR(64) NOT NULL
);