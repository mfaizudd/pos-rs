table! {
    products (id) {
        id -> Uuid,
        name -> Varchar,
        barcode -> Nullable<Varchar>,
        price -> Numeric,
        stock -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    transaction_products (transaction_id, user_id) {
        transaction_id -> Uuid,
        user_id -> Uuid,
        quantity -> Int4,
        price -> Nullable<Numeric>,
    }
}

table! {
    transactions (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        full_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(transaction_products -> transactions (transaction_id));
joinable!(transaction_products -> users (user_id));
joinable!(transactions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    products,
    transaction_products,
    transactions,
    users,
);
