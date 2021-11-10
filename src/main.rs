#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
/*
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client, Error, Region, PKG_VERSION};
use std::path::Path;
*/
mod api;
mod models;
mod schema;
mod utils;

#[launch]
fn rocket() -> _ {
    /*
       let region_provider = RegionProviderChain::first_try(Region::new("ap-northeast-1"))
           .or_default_provider()
           .or_else(Region::new("ap-northeast-1"));

       let shared_config = aws_config::from_env().region(region_provider).load().await;

       let client = Client::new(&shared_config);
       let body = ByteStream::from_path(Path::new("Cargo.toml")).await;

       let resp = client
           .put_object()
           .bucket("rusty-crab-cookpad-dev")
           .key("test-text")
           .body(body.unwrap())
           .send()
           .await;

       println!("[resp]: {:?}", resp);
    */
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
                api::user::show_browsed_recipes,
                api::user::whether_like_recipe,
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
