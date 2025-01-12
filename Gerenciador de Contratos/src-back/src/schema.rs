// @generated automatically by Diesel CLI.

diesel::table! {
    codigos_recuperacao (idcodigo) {
        #[max_length = 6]
        codigo -> Varchar,
        datacriacao -> Timestamp,
        dataexpiracao -> Timestamp,
        #[max_length = 32]
        status -> Varchar,
        #[max_length = 64]
        idusuario -> Varchar,
        #[max_length = 64]
        idcodigo -> Varchar,
    }
}

diesel::table! {
    imagens (idimagem) {
        #[max_length = 64]
        idimagem -> Varchar,
        #[max_length = 128]
        nome -> Varchar,
        bin -> Bytea,
        #[max_length = 128]
        link -> Varchar,
    }
}

diesel::table! {
    imagens_maquinas (idimagemmaquina) {
        #[max_length = 64]
        idimagemmaquina -> Varchar,
        #[max_length = 64]
        idimagem -> Varchar,
        #[max_length = 64]
        idmaquina -> Varchar,
    }
}

diesel::table! {
    maquinas (idmaquina) {
        #[max_length = 64]
        idmaquina -> Varchar,
        #[max_length = 64]
        idpublico -> Varchar,
        #[max_length = 64]
        nome -> Varchar,
        #[max_length = 64]
        numeroserie -> Varchar,
        #[max_length = 64]
        categoria -> Varchar,
        valoraluguel -> Float8,
        #[max_length = 64]
        disponivelaluguel -> Varchar,
        #[max_length = 64]
        status -> Varchar,
        datacadastro -> Timestamp,
        dataatualizacao -> Timestamp,
        #[max_length = 64]
        descricao -> Varchar,
    }
}

diesel::table! {
    usuarios (idusuario) {
        #[max_length = 64]
        nome -> Varchar,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 64]
        senha -> Varchar,
        #[max_length = 64]
        documento -> Varchar,
        datacadastro -> Timestamp,
        #[max_length = 64]
        idusuario -> Varchar,
    }
}

diesel::joinable!(codigos_recuperacao -> usuarios (idusuario));

diesel::allow_tables_to_appear_in_same_query!(
    codigos_recuperacao,
    imagens,
    imagens_maquinas,
    maquinas,
    usuarios,
);
