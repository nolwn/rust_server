use diesel;
use diesel::prelude::*;
use rust_server::schema::movies;
use rust_server::models::Movie;
use rust_server::establish_connection;
// use self::schema::movies::dls::*;

/*
 *  Below are the routes!
 */


#[get("/movies")]
pub fn movies() -> String {
    let connection = establish_connection();

    let movies = movies::table.load::<Movie>(&connection);
    // serde_json::to_string(&movies).unwrap()
    "".to_string()
}

#[get("/movies/<id>")] // <id> is a parameter, in this case representing an id
pub fn movie(id: String) -> String {
    format!("You are looking for a movie with and id of {}", id)
}
