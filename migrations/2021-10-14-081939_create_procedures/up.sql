CREATE TABLE procedures (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL,
    number INTEGER NOT NULL,
    discription VARCHAR NOT NULL,
    image_path VARCHAR
);