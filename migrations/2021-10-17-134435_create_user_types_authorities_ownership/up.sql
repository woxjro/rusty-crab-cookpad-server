CREATE TABLE user_types_authorities_ownership (
    id SERIAL PRIMARY KEY,
    user_type_id INTEGER NOT NULL,
    authority_id INTEGER NOT NULL
);