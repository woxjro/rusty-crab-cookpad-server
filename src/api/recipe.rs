use crate::api::MyDatabase;
use crate::models;
use futures::future::join_all;
use json_patch::merge;
use models::ingredient::Ingredient;
use models::procedure::Procedure;
use models::recipe::{Recipe, RecipeWithItems};
use rocket::serde::json::{json, Json, Value};

#[get("/<id>")]
pub async fn read_recipe(conn: MyDatabase, id: usize) -> Json<RecipeWithItems> {
    let recipe = conn.run(move |c| Recipe::from(c, id as i32)).await;
    let ingredients = conn.run(move |c| Ingredient::from(c, id as i32)).await;
    let procedures = conn.run(move |c| Procedure::from(c, id as i32)).await;
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
    })
}

#[get("/")]
pub async fn show_recipes(conn: MyDatabase) -> Json<Vec<RecipeWithItems>> {
    let mut res = vec![];
    let recipes = conn.run(|c| Recipe::read(c)).await;
    for recipe in recipes {
        let id = recipe.id;
        let ingredients = conn.run(move |c| Ingredient::from(c, id as i32)).await;
        let procedures = conn.run(move |c| Procedure::from(c, id as i32)).await;
        let r = RecipeWithItems {
            id: recipe.id,
            user_id: recipe.user_id,
            title: recipe.title,
            thumbnail_path: recipe.thumbnail_path,
            created_at: recipe.created_at,
            updated_at: recipe.updated_at,
            discription: recipe.discription,
            ingredients: ingredients,
            procedures: procedures,
        };
        res.push(r);
    }
    Json(res)
}
