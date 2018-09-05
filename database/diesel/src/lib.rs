#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use std::env;

use diesel::prelude::*;
use dotenv::dotenv;

use self::models::{NewPost, Post};

// Wrapping Postgres connection
#[cfg(feature = "postgres")]
type MyConnection = diesel::pg::PgConnection;

#[cfg(feature = "postgres")]
pub fn establish_connection() -> MyConnection {
    dotenv().ok();

    let database_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");

    diesel::pg::PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &MyConnection, title: &'a str, body: &'a str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost {
        title: title.to_string(),
        body: body.to_string(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

#[cfg(test)]
mod tests {
    use diesel::sql_query;

    use super::models::*;
    use super::schema::*;
    use super::*;

    #[test]
    fn belongs_to() -> QueryResult<()> {
        let connection = establish_connection();
        connection.begin_test_transaction()?;

        connection.execute(
            "INSERT INTO users (id, name) VALUES 
                (1, 'Sean'), 
                (2, 'Tess')",
        )?;
        connection.execute(
            "INSERT INTO posts (id, user_id, title, body) VALUES
                (1, 1, 'Hello', 'Content'),
                (2, 2, 'World', 'Content')",
        )?;

        let sean = User::new(1, "Sean");
        let tess = User::new(2, "Tess");
        let seans_post = Post::new(1, 1, "Hello", "Content");
        let tess_post = Post::new(2, 2, "World", "Content");

        let expected_data = vec![(seans_post, sean), (tess_post, tess)];
        let actual_data = posts::table.inner_join(users::table).load(&connection)?;

        assert_eq!(expected_data, actual_data);

        return Ok(());
    }

    #[test]
    fn raw_sql() -> QueryResult<()> {
        let connection = establish_connection();
        connection.begin_test_transaction()?;

        connection.execute(
            "INSERT INTO users (id, name) VALUES 
                (1, 'Sean'), 
                (2, 'Tess')",
        )?;
        connection.execute(
            "INSERT INTO posts (id, user_id, title, body) VALUES
                (1, 1, 'Hello', 'Content'),
                (2, 2, 'World', 'Content')",
        )?;

        let sean = User::new(1, "Sean");
        let tess = User::new(2, "Tess");
        let expected_data = vec![sean, tess];

        let actual_data = sql_query("select * from users").get_results(&connection)?;

        assert_eq!(expected_data, actual_data);

        let seans_post = NewPost {
            title: "Hello".to_string(),
            body: "Content".to_string(),
        };
        let tess_post = NewPost {
            title: "World".to_string(),
            body: "Content".to_string(),
        };
        let expected_data = vec![seans_post, tess_post];

        let actual_data = sql_query("select body, title from posts").get_results(&connection)?;

        assert_eq!(expected_data, actual_data);

        return Ok(());
    }
}
//[cfg(test)]
