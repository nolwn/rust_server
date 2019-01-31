// These are structs which are used to define how things get inserted into, or
// pulled out from, the database.

// use std::time::{ DateTime };
use rocket::request::{Form, FromFormValue};
use rocket::http::RawStr;
use super::schema::movies;
use super::schema::actors;
use serde_derive::*;
use chrono::prelude::*;

pub struct DateWrap(NaiveDateTime);

#[derive(Queryable, Serialize, Deserialize, Debug)] // Tells Diesel that this is returned from SELECT
pub struct Movie {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub release_date: i32,
    pub rating: String,
    pub poster_url: Option<String>
}

#[derive(Insertable, Serialize, Deserialize, Debug, FromForm)] // Tells Diesel that this is added with INSERT
#[table_name = "movies"]
pub struct NewMovie { // CamelCase for a new Type...
    pub name: String,
    pub description: String,
    pub release_date: i32,
    pub rating: String,
    pub poster_url: Option<String>
}

#[derive(Insertable)]
#[table_name = "actors"]
pub struct Actor {
    pub id: i32,
    pub name: String,
    pub biography: String

}
