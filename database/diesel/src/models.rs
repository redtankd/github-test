use super::schema::posts;
use super::schema::users;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, QueryableByName, Identifiable, Insertable, AsChangeset,
)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

impl User {
    pub fn new(id: i32, name: &str) -> Self {
        User {
            id: id,
            name: name.to_string(),
            hair_color: None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
}

impl Post {
    pub fn new(id: i32, user_id: i32, title: &str, body: &str) -> Self {
        Post {
            id: id,
            user_id: user_id,
            title: title.to_string(),
            body: body.to_string(),
            published: false,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Insertable, QueryableByName)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

// select doesn't support &str
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }
