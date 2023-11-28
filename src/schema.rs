// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}
