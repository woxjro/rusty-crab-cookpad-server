use crate::api::MyDatabase;
use crate::models;
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
