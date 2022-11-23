// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Int4,
        name -> Varchar,
        tel_number -> Varchar,
        role -> Role,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
