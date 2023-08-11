-- Your SQL goes here
CREATE TABLE IF NOT EXISTS likes (
  id Varchar PRIMARY KEY NOT NULL,
  created_at TIMESTAMP DEFAULT now() NOT NULL,
  tweet_id Varchar NOT NULL REFERENCES tweets (id)
);