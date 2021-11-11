use crate::api::MyDatabase;
use crate::models;
use models::tag::Tag;
use rocket::serde::json::Json;

#[get("/")]
pub async fn read(conn: MyDatabase) -> Json<Vec<Tag>> {
    let tags = conn.run(move |c| Tag::read(c)).await;
    Json(tags)
}
