use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::access_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccessToken {
    pub id: i32,
    pub token: String,
    pub status: Option<bool>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::entities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Entity {
    pub id: i32,
    pub name: String,
    pub folder_path: String,
    pub store_type: Option<String>,
    pub observation: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct File {
    pub id: i32,
    pub entity_id: i32,
    pub file_name: String,
    pub size: i64,
    pub sha256: String,
    pub is_sha256_valid: Option<bool>,
    pub is_restored: Option<bool>,
}
