/*
 *  Below are the routes!
 */
#[get("/movies")]
pub fn movies() -> &'static str {
    "movies!"
}

#[get("/movies/<id>")] // <id> is a parameter, in this case representing an id
pub fn movie(id: String) -> String {
    format!("You are looking for a movie with and id of {}", id)
}
