CREATE TABLE ingredients (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    amount VARCHAR NOT NULL
);