extern crate diesel;

use super::schema::posts;
use serde::{Serialize, Deserialize};
use chrono::naive::serde::ts_seconds;

#[derive(Queryable, Debug, Serialize, Deserialize, QueryableByName)]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    #[serde(with = "ts_seconds")]
    pub create_time: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name="posts"]
pub struct NewPost {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[table_name="posts"]
pub struct UpdatePost {
    pub title: String,
    pub body: String,
    // pub published: bool
}