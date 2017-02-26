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
    use super::*;
    use super::models::*;
    use super::schema::*;

    #[test]
    fn belongs_to() {
        let connection = establish_connection();
        connection.begin_test_transaction().unwrap();

        connection
            .execute("INSERT INTO users (id, name) VALUES 
                (1, 'Sean'), 
                (2, 'Tess')")
            .unwrap();
        connection
            .execute("INSERT INTO posts (id, user_id, title, body) VALUES
                (1, 1, 'Hello', 'Content'),
                (2, 2, 'World', 'Content')")
            .unwrap();

        let sean = User::new(1, "Sean");
        let tess = User::new(2, "Tess");
        let seans_post = Post::new(1, 1, "Hello", "Content");
        let tess_post = Post::new(2, 2, "World", "Content");

        let expected_data = vec![(seans_post, sean), (tess_post, tess)];
        let source = posts::table.inner_join(users::table);
        let actual_data: Vec<_> = source.load(&connection).unwrap();

        assert_eq!(expected_data, actual_data);
    }
}
//[cfg(test)]