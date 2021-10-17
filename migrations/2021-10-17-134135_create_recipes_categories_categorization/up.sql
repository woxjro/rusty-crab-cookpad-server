CREATE TABLE recipes_categories_categorization (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL
);