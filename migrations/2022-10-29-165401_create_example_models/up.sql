CREATE TABLE example_models (
  id SERIAL PRIMARY KEY,
  label TEXT NOT NULL,
  label_nullable TEXT,
  label_array TEXT[] NOT NULL,
  label_array_nullable TEXT[]
);
