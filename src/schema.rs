// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        street -> Varchar,
        city -> Varchar,
        zip_code -> Varchar,
        state -> Varchar,
    }
}
