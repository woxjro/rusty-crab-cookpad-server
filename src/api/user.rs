use crate::api::MyDatabase;
use crate::models;
use models::recipe::Recipe;
use models::user::{NewUser, User, UserWithAuthorities};
use rocket::serde::json::{json, Json, Value};

#[get("/<id>")]
pub async fn read_user(conn: MyDatabase, id: usize) -> Json<UserWithAuthorities> {
    let user = conn.run(move |c| User::from(c, id as i32)).await;
    Json(user)
}

#[delete("/<id>")]
pub async fn delete_user(conn: MyDatabase, id: usize) -> Value {
    let done = conn.run(move |c| User::delete(id as i32, c)).await;

    if done {
        json!({ "status": "ok", "id": id })
    } else {
        json!({ "status": "error", "id": id })
    }
}

#[post("/", format = "json", data = "<user>")]
pub async fn create_user(conn: MyDatabase, user: Json<NewUser>) -> Json<User> {
    let res = conn.run(move |c| User::create(user.into_inner(), c)).await;
    Json(res)
}

#[get("/")]
pub async fn show_users(conn: MyDatabase) -> Json<Vec<UserWithAuthorities>> {
    let res = conn.run(|c| User::read(c)).await;
    Json(res)
}

#[get("/whether_like_recipe/<user_id>/<recipe_id>")]
pub async fn whether_like_recipe(conn: MyDatabase, user_id: usize, recipe_id: usize) -> Json<bool> {
    let res = conn
        .run(move |c| User::is_recipe_liked(c, user_id as i32, recipe_id as i32))
        .await;
    Json(res)
}

#[get("/liked_recipes/<user_id>")]
pub async fn show_liked_recipes(conn: MyDatabase, user_id: usize) -> Json<Vec<Recipe>> {
    let res = conn
        .run(move |c| User::liked_recipes(c, user_id as i32))
        .await;
    Json(res)
}

#[get("/browsed_recipes/<user_id>")]
pub async fn show_browsed_recipes(conn: MyDatabase, user_id: usize) -> Json<Vec<Recipe>> {
    let res = conn
        .run(move |c| User::recipes_browsing_history(c, user_id as i32))
        .await;
    Json(res)
}
