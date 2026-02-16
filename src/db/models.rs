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
#[diesel(table_name = crate::db::schema::entity)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Entity {
    pub id: i32,
    pub name: String,
    pub folder_path: String,
}
