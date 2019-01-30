CREATE TABLE token_key_map (
  id SERIAL PRIMARY KEY,
  token TEXT NOT NULL UNIQUE,
  key TEXT NOT NULL
);

CREATE TABLE links (
  id SERIAL PRIMARY KEY,
  owner TEXT REFERENCES token_key_map (token),
  origin TEXT NOT NULL UNIQUE,
  dest TEXT NOT NULL,
  creation_date TIMESTAMP NOT NULL DEFAULT current_timestamp,
  last_used TIMESTAMP,
  clicks INTEGER DEFAULT 0 NOT NULL,
  expire_date TIMESTAMP,
  expire_clicks INTEGER
);