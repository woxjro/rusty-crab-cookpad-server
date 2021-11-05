#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod api;
mod models;
mod schema;
mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(api::MyDatabase::fairing())
        .attach(utils::cors::CORS)
        .mount(
            "/api/user",
            routes![
                api::user::show_users,
                api::user::create_user,
                api::user::read_user,
                api::user::delete_user,
                api::user::show_liked_recipes,
                api::user::show_browsed_recipes
            ],
        )
        .mount(
            "/api/recipe",
            routes![
                api::recipe::show_recipes,
                api::recipe::read_recipe,
                api::recipe::show_recipes_with_query,
                api::recipe::search,
                api::recipe::like_recipe,
                api::recipe::unlike_recipe,
            ],
        )
}
