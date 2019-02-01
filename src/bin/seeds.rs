use diesel::prelude::*;
use rust_server::*;
use rust_server::movie_models::*;
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
        poster_url: Option::Some("butt".to_string()),
        release_date: 1982,
        rating: "I don't understand what this is".to_string()
    };

    #[derive(Insertable)]
    let new_movies = vec![
        NewMovie {
            name: "The Thing".to_string(),
            description: "Scientists in anartica discover something ominous under the ice.".to_string(),
            poster_url: Option::Some("butt".to_string()),
            release_date: 1982,
            rating: "I don't understand what this is".to_string()
        },
        NewMovie {
            name: "The Dark Knight".to_string(),
            description: "Batman gets all gritty and angsty with the joker".to_string(),
            poster_url: Option::None,
            release_date: 2008,
            rating: "It was pretty schweet".to_string()
        },
        NewMovie {
            name: "The Conversion".to_string(),
            description: "Gene Hackman does badass things and it's all cray".to_string(),
            poster_url: Option::None,
            release_date: 1974,
            rating: "Great movie, I should watch it again soon.".to_string()
        },
        NewMovie {
            name: "Free Solo".to_string(),
            description: "A dumbass who should wear ropes when climbing does not. I hate him.".to_string(),
            poster_url: Option::Some("no".to_string()),
            release_date: 1982,
            rating: "He needs to wear a fucking rope. But the movie was good.".to_string()
        },
        NewMovie {
            name: "Star Wars".to_string(),
            description: "Luke Skywalker wants to be where the people are.".to_string(),
            poster_url: Option::Some("butt".to_string()),
            release_date: 1982,
            rating: "Fucking. Bad. Ass.".to_string()
        }
    ];

    diesel::insert_into(movies)
        .values(new_movies)
        .execute(&connection)
        .expect("Could not insert");
}
