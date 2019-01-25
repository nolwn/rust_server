// These are features required by Rocket that are contained in the "Nightly" build
#![feature(proc_macro_hygiene, decl_macro)]

// This is Rocket! I don't know what "macro_use" means, but it's how you import it so... cool.
#[macro_use] extern crate rocket;
extern crate postgres;

use postgres::{ connection, TlsMode }

mod movie_routes;

#[get("/actors")]
fn actors() -> &'static str {
    "actors!"
}

// 404 error handler
#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
    format!("{} is not valid.", req.uri())
}

// Main function wraps everything up and launches Rocket
fn main() {
    movie_routes::movies();
    rocket::ignite() // Prep for launch...
        .mount("/", routes![movie_routes::movies, movie_routes::movie, actors]) // Pull in the routes...
        .register(catchers![not_found]) // Pull in the error handlers...
        .launch(); // LAUNCH!!!!
}
