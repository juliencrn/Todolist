use super::schema::posts;
use super::schema::tasks;
// use diesel::data_types::Timestamp;
// use diesel::prelude::*;
// use diesel::sql_types::*;
// use serde::Deserialize;
// use serde::Serialize;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: &'a i32,
}

#[derive(Queryable, Debug)]
pub struct Task {
    pub id: i32,
    pub text: String,
    // pub created_at: chrono::NaiveDateTime,
    pub completed: i32,
    // pub created_at: String,
}

#[derive(Insertable, Debug)]
#[table_name = "tasks"]
pub struct NewTask {
    pub text: String,
    pub completed: i32,
}
