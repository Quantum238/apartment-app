table! {
    users (id) {
        id -> Int4,
        password -> Varchar,
        email -> Varchar,
        created -> Timestamptz,
        modified -> Timestamptz,
        settings -> Nullable<Jsonb>,
    }
}
