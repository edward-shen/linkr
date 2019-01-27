-- Your SQL goes here

CREATE TABLE links (
  id SERIAL PRIMARY KEY,
  owner INTEGER REFERENCES users(id),
  origin TEXT NOT NULL,
  dest TEXT NOT NULL,
  is_private BOOL NOT NULL,
  clicks INTEGER DEFAULT 0 NOT NULL
)