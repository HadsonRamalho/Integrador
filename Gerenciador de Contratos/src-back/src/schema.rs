// @generated automatically by Diesel CLI.

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
        #[max_length = 64]
        idusuario -> Varchar,
    }
}
