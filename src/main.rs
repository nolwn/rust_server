#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/movies")]
fn movies() -> &'static str {
    "movies!"
}

#[get("/movies/<id>")]
fn movie(id: String) -> String {
    format!("You are looking for a movie with and id of {}", id)
}

#[get("/actors")]
fn actors() -> &'static str {
    "actors!"
}

// #[]

fn main() {
    rocket::ignite().mount("/", routes![movies, movie, actors]).launch();
}
