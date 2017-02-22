#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

pub mod schema;
pub mod models;

use std::env;

use diesel::prelude::*;
use dotenv::dotenv;

use self::models::{Post, NewPost};

// Wrapping Postgres connection
#[cfg(feature = "postgres")]
type MyConnection = diesel::pg::PgConnection;

#[cfg(feature = "postgres")]
pub fn establish_connection() -> MyConnection {
    dotenv().ok();

    let database_url = env::var("POSTGRES_URL")
        .expect("POSTGRES_URL must be set");

    diesel::pg::PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &MyConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result(conn)
        .expect("Error saving new post")
}

#[cfg(test)]
mod tests {

}