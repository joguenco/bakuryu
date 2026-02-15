use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

use crate::DBPool;

pub fn connect() -> DBPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("failed to get DB Url");
    let db = ConnectionManager::new(db_url);

    r2d2::Pool::builder()
        .max_size(5)
        .build(db)
        .expect("failed to create db pool")
}
