use crate::api::MyDatabase;
use crate::models;
use models::category::Category;
use models::ingredient::Ingredient;
use models::procedure::Procedure;
use models::recipe::{Recipe, RecipeWithItems};
use models::tag::Tag;
use models::user::User;
use rocket::serde::json::Json;

#[get("/<recipe_id>/by/<user_id>")]
pub async fn read_recipe(
    conn: MyDatabase,
    recipe_id: usize,
    user_id: usize,
) -> Json<RecipeWithItems> {
    let recipe = conn.run(move |c| Recipe::from(c, recipe_id as i32)).await;
    let ingredients = conn
        .run(move |c| Ingredient::from(c, recipe_id as i32))
        .await;
    let procedures = conn
        .run(move |c| Procedure::from(c, recipe_id as i32))
        .await;
    let tags = conn.run(move |c| Tag::from(c, recipe_id as i32)).await;
    let categories = conn.run(move |c| Category::from(c, recipe_id as i32)).await;

    conn.run(move |c| User::browse_recipe(c, user_id as i32, recipe_id as i32))
        .await;

    Json(RecipeWithItems {
        id: recipe.id,
        user_id: recipe.user_id,
        title: recipe.title,
        thumbnail_path: recipe.thumbnail_path,
        created_at: recipe.created_at,
        updated_at: recipe.updated_at,
        discription: recipe.discription,
        ingredients: ingredients,
        procedures: procedures,
        tags: tags,
        categories: categories,
    })
}

#[get("/")]
pub async fn show_recipes(conn: MyDatabase) -> Json<Vec<RecipeWithItems>> {
    let recipes = conn.run(|c| Recipe::read(c)).await;
    Json(recipes)
}

#[post("/<recipe_id>/is_liked_by/<user_id>")]
pub async fn like_recipe(conn: MyDatabase, recipe_id: usize, user_id: usize) -> Json<String> {
    let ok = conn
        .run(move |c| User::like_recipe(c, user_id as i32, recipe_id as i32))
        .await;

    match ok {
        true => Json("success".to_string()),
        false => Json("failed".to_string()),
    }
}

#[post("/<recipe_id>/is_unliked_by/<user_id>")]
pub async fn unlike_recipe(conn: MyDatabase, recipe_id: usize, user_id: usize) -> Json<String> {
    let ok = conn
        .run(move |c| User::unlike_recipe(c, user_id as i32, recipe_id as i32))
        .await;
    match ok {
        true => Json("success".to_string()),
        false => Json("failed".to_string()),
    }
}

#[get("/search?<words>")]
pub async fn search(conn: MyDatabase, words: String) -> Json<Vec<Recipe>> {
    let words = words.split(" ").map(|word| word.to_string()).collect();
    let recipes = conn.run(|c| Recipe::search(c, words)).await;
    Json(recipes)
}

#[get("/query?<user_id>&<category_id>&<tag_id>")]
pub async fn show_recipes_with_query(
    conn: MyDatabase,
    user_id: Option<i32>,
    category_id: Option<i32>,
    tag_id: Option<i32>,
) -> Json<Vec<RecipeWithItems>> {
    let recipes = conn
        .run(move |c| Recipe::read_with_query(c, user_id, tag_id, category_id))
        .await;

    Json(recipes)
}
