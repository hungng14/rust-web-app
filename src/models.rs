extern crate diesel;

use super::schema::posts;
use serde::{Serialize, Deserialize};
#[derive(Queryable, Debug, Serialize, Deserialize, QueryableByName)]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[table_name="posts"]
pub struct UpdatePost {
    pub title: String,
    pub body: String,
    pub published: bool
}