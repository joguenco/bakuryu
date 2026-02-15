diesel::table! {
    access_tokens (id) {
        id -> Int4,
        token -> Varchar,
        status -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
