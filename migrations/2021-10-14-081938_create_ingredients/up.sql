CREATE TABLE ingredients (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER,
    name VARCHAR,
    amount VARCHAR
);