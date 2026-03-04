// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (id) {
        id -> Int4,
        token -> Varchar,
        status -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    entities (id) {
        id -> Int4,
        access_token_id -> Int4,
        name -> Varchar,
        folder_path -> Varchar,
        store_type -> Nullable<Varchar>,
        observation -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    files (id) {
        id -> Int4,
        entity_id -> Int4,
        file_name -> Varchar,
        size -> Int8,
        sha256 -> Varchar,
        is_sha256_valid -> Nullable<Bool>,
        is_restored -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(entities -> access_tokens (access_token_id));
diesel::joinable!(files -> entities (entity_id));

diesel::allow_tables_to_appear_in_same_query!(access_tokens, entities, files,);
