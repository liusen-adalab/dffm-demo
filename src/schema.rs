// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        tel_number -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
