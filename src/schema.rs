table! {
    actors (id) {
        id -> Int4,
        name -> Varchar,
        biography -> Text,
        profile_url -> Nullable<Varchar>,
        birth_date -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    actors_movies (id) {
        id -> Int4,
        movie_id -> Int4,
        actor_id -> Int4,
        role -> Varchar,
    }
}

table! {
    movies (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        release_date -> Timestamp,
        rating -> Varchar,
        poster_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(actors_movies -> actors (actor_id));
joinable!(actors_movies -> movies (movie_id));

allow_tables_to_appear_in_same_query!(
    actors,
    actors_movies,
    movies,
);
