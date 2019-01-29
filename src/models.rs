// These are structs which are used to define how things get inserted into, or
// pulled out from, the database.


// use std::time::{ DateTime };
use super::schema::movies;
use super::schema::actors;
use serde_derive::*;
use chrono::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug)] // Tells Diesel that this is returned from SELECT
pub struct Movie {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub release_date: NaiveDateTime,
    pub rating: String,
    pub poster_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)] // Tells Diesel that this is added with INSERT
#[table_name = "movies"]
pub struct NewMovie { // CamelCase for a new Type...
    pub name: String,
    pub description: String,
    pub release_date: NaiveDateTime,
    pub rating: String,
    // pub poster_url: Option<String>,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name = "actors"]
pub struct Actor {
    pub id: i32,
    pub name: String,
    pub biography: String,

}
