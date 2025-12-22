-- Your SQL goes here
CREATE TABLE codigos_recuperacao(    
    codigo VARCHAR(6) NOT NULL,
    datacriacao TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    dataexpiracao TIMESTAMP NOT NULL,
    status VARCHAR(32) NOT NULL,
    idusuario VARCHAR(64) NOT NULL,
    idcodigo VARCHAR(64) PRIMARY KEY,
    CONSTRAINT fk_usuarios FOREIGN KEY (idusuario) REFERENCES usuarios(idusuario) ON DELETE CASCADE
);