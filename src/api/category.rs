use crate::api::MyDatabase;
use crate::models;
use models::category::Category;
use rocket::serde::json::Json;

#[get("/")]
pub async fn read(conn: MyDatabase) -> Json<Vec<Category>> {
    let categories = conn.run(move |c| Category::read(c)).await;
    Json(categories)
}
