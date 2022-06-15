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
