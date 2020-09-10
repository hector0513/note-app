-- Your SQL goes here
CREATE TABLE notes (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  created timestamp  NOT NULL
)