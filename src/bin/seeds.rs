use diesel::prelude::*;
use rust_server::*;
use rust_server::models::*;
use std::time::{ Duration, SystemTime };

fn main() {
    use schema::actors::dsl::*;
    use schema::movies::dsl::*;

    let connection = establish_connection();

    diesel::delete(movies).execute(&connection).expect("Error deleteing posts");
    diesel::delete(actors).execute(&connection).expect("Error deleteing users");

    let new_movie = NewMovie {
            name: "The Thing".to_string(),
            description: "Scientists in anartica discover something ominous under the ice.".to_string(),
            release_date: SystemTime::now(),
            rating: "I don't understand what this is".to_string()
        };

    diesel::insert_into(movies)
        .values(&new_movie)
        .execute(&connection)
        .expect("Could not insert");
}



// id INTEGER PRIMARY KEY,
// name VARCHAR (255) NOT NULL,
// description TEXT NOT NULL,
// release_date TIMESTAMP NOT NULL,
// rating VARCHAR (255) NOT NULL,
// poster_url VARCHAR (255),
// created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
// updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
