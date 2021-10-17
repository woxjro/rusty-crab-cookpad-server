CREATE TABLE recipes_tags_tagging (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL
);