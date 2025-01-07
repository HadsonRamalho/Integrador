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
    maquinas (idmaquina) {
        #[max_length = 64]
        idmaquina -> Varchar,
        #[max_length = 64]
        idpublico -> Varchar,
        #[max_length = 64]
        nome -> Varchar,
        #[max_length = 64]
        numeroserie -> Varchar,
        valoraluguel -> Float8,
        #[max_length = 64]
        disponivelaluguel -> Varchar,
        #[max_length = 64]
        status -> Varchar,
        datacadastro -> Timestamp,
        dataatualizacao -> Timestamp,
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
    maquinas,
    usuarios,
);
