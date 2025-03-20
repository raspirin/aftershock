use diesel::prelude::*;
use dotenvy::dotenv;
use models::{Content, ContentKind, NewContent};
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is expected.");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Can not establish database connection."))
}

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Content {
    use crate::schema::contents;

    let new_content = NewContent::new(ContentKind::Post, title, body);

    // FIXME: better error handling ie. return error to caller and let frontend know
    diesel::insert_into(contents::table)
        .values(&new_content)
        .returning(Content::as_returning())
        .get_result(conn)
        .expect("Error saving new post.")
}
