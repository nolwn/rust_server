
use diesel;
use diesel::prelude::*;
use rust_server::schema::movies;
use rust_server::movie_models::*;
use rust_server::establish_connection;
use rust_server::schema::movies::dsl::*;
use rocket_contrib::json::Json;

/*
 *  Below are the routes!
 */


#[get("/movies")]
pub fn movies() -> String {
    let connection = establish_connection();

    let movies = movies::table.load::<Movie>(&connection).ok();

    serde_json::to_string(&movies).unwrap()
    // format!("{:?}", movies)
    // "".to_string()
}

#[get("/movies/<m_id>")] // <id> is a parameter, in this case representing an id
pub fn movie(m_id: i32) -> String {
    let connection = establish_connection();

    let movie = movies::table
        .find(m_id).first::<Movie>(&connection)
        .ok()
        .unwrap();

    serde_json::to_string(&movie)
        .unwrap()
        .expect("NO!".to_string())
}

#[post("/movies", data="<body>")]
pub fn create_movie(body: Json<NewMovie>) -> String {
    use rust_server::schema::movies::dsl::*;

    let connection = establish_connection();

    let new_movie = body.into_inner();

    diesel::insert_into(movies)
        .values(&new_movie)
        .execute(&connection)
        .expect("Could not insert");

    format!("Added the thing")
}
