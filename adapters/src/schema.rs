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

diesel::table! {
    orders (id) {
        id -> Uuid,
        customer_id -> Uuid,
    }
}

diesel::allow_tables_to_appear_in_same_query!(customers, orders,);
