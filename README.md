# Rusty Crab Cookpad Server

`cargo watch -x run`

https://diesel.rs/guides/getting-started
- `diesel setup`
- `diesel migration generate create_hogehoges`
- `diesel migration run`

`rustup toolchain list`
`rustup override add nightly`


```
diesel migration generate create_users &&
diesel migration generate create_recipes  &&
diesel migration generate create_comments &&
diesel migration generate create_user_types &&
diesel migration generate create_authorities &&
diesel migration generate create_ingredients &&
diesel migration generate create_procedures &&
diesel migration generate create_tags &&
diesel migration generate create_categories

```