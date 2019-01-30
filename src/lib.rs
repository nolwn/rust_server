#[macro_use] extern crate diesel; // The ORM and query builder for this project
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate dotenv; // loads env variables from a .env file

use diesel::prelude::*; // prelude is way of importing a useful set of diesel in one line
use diesel::pg::PgConnection; // the postgres connection type
use dotenv::dotenv; // load env variables from the .env file
use std::env; // access to the environment variables

pub mod schema; // a bunch of macros that Diesel generates for me.
pub mod movie_models; // structs that describe the shape of my queries and their responses

// a function with no parameters that returns a PgConnection, a connection to Postgresql
pub fn establish_connection() -> PgConnection {

    // the dotenv() function loads the variables and returns a Result enum, which is a built
    // in type in Rust. Result can be either be some type, or it can be an error. The ok()
    // function returns the success type. I think the assumption here is that this won't fail.
    dotenv().ok();

    // env is a module which is a collection of datastructures and functions and whatnot
    // (it's a way of loading external things) var() is a function that takes a &str and
    // looks it up in the environment. It returns a Result enum, which I described above.
    let database_url = env::var("DATABASE_URL")

        // Expect is like catch in JS Promise, if there is an error it catches it and
        // throws(?) this message.
        .expect("The DATABASE_URL variable needs to be set");

    // It's a litte bit of a guess... but I think the & is converting a String to a str.
    // On further study that is what is happeneing but this kind of coercion relies on the
    // fact that the function takes a &str and you pass it a &String then the rustc knows to
    // make the conversion. Normally if you had a string and put the & in font of it you would
    // get a refernce to the String type—a &String.
    PgConnection::establish(&database_url)
        .expect(&format!("Could not connect to {}", database_url))


}
