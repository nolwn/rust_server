CREATE TABLE movies (
  id SERIAL PRIMARY KEY,
  name VARCHAR (255) NOT NULL,
  description TEXT NOT NULL,
  release_date INTEGER NOT NULL,
  rating VARCHAR (255) NOT NULL,
  poster_url VARCHAR (255)
);
