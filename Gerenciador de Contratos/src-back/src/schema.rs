// @generated automatically by Diesel CLI.

diesel::table! {
    codigos_recuperacao (idcodigo) {
        #[max_length = 12]
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
    contas_bancarias (idconta) {
        #[max_length = 64]
        idconta -> Varchar,
        #[max_length = 64]
        idusuario -> Varchar,
        #[max_length = 64]
        numeroconta -> Varchar,
        #[max_length = 64]
        numeroagencia -> Varchar,
        #[max_length = 64]
        nomebanco -> Varchar,
    }
}

diesel::table! {
    contratos (idcontrato) {
        #[max_length = 64]
        idcontrato -> Varchar,
        #[max_length = 64]
        idlocatario -> Varchar,
        #[max_length = 64]
        idlocador -> Varchar,
        #[max_length = 64]
        idenderecolocatario -> Varchar,
        #[max_length = 64]
        idenderecolocador -> Varchar,
        #[max_length = 64]
        idenderecoretirada -> Varchar,
        #[max_length = 64]
        idmaquina -> Varchar,
        #[max_length = 64]
        prazolocacao -> Varchar,
        #[max_length = 64]
        medidatempolocacao -> Varchar,
        #[max_length = 64]
        valorlocacao -> Varchar,
        #[max_length = 64]
        contabancarialocador -> Varchar,
        #[max_length = 64]
        cidadeforo -> Varchar,
    }
}

diesel::table! {
    enderecos (idendereco) {
        #[max_length = 64]
        idendereco -> Varchar,
        #[max_length = 64]
        pais -> Varchar,
        #[max_length = 64]
        estado -> Varchar,
        #[max_length = 64]
        cidade -> Varchar,
        #[max_length = 12]
        cep -> Varchar,
        #[max_length = 64]
        bairro -> Varchar,
        #[max_length = 64]
        logradouro -> Varchar,
        #[max_length = 64]
        numero -> Varchar,
        #[max_length = 64]
        complemento -> Varchar,
    }
}

diesel::table! {
    enderecos_usuarios (idenderecousuario) {
        #[max_length = 64]
        idenderecousuario -> Varchar,
        #[max_length = 64]
        idendereco -> Varchar,
        #[max_length = 64]
        idusuario -> Varchar,
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
    locadoras (idlocadora) {
        #[max_length = 64]
        idlocadora -> Varchar,
        #[max_length = 64]
        idusuario -> Varchar,
        #[max_length = 64]
        idendereco -> Varchar,
        #[max_length = 64]
        idconta -> Varchar,
    }
}

diesel::table! {
    locatarios (idlocatario) {
        #[max_length = 64]
        idlocatario -> Varchar,
        #[max_length = 64]
        idusuario -> Varchar,
        #[max_length = 64]
        idendereco -> Varchar,
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
    maquinas_usuarios (idmaquinausuario) {
        #[max_length = 64]
        idmaquinausuario -> Varchar,
        #[max_length = 64]
        idmaquina -> Varchar,
        #[max_length = 64]
        idusuario -> Varchar,
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
        #[max_length = 64]
        origemconta -> Nullable<Varchar>,
    }
}

diesel::joinable!(codigos_recuperacao -> usuarios (idusuario));

diesel::allow_tables_to_appear_in_same_query!(
    codigos_recuperacao,
    contas_bancarias,
    contratos,
    enderecos,
    enderecos_usuarios,
    imagens,
    imagens_maquinas,
    locadoras,
    locatarios,
    maquinas,
    maquinas_usuarios,
    usuarios,
);
