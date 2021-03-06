use crate::api::MyDatabase;
use crate::models;
use crate::models::ingredient::NewIngredient;
use models::category::Category;
use models::ingredient::Ingredient;
use models::procedure::{NewProcedure, Procedure};
use models::recipe::{NewRecipe, NewRecipeWithItems, Recipe, RecipeWithItems, UpdateRecipe};
use models::tag::Tag;
use models::user::User;
use models::user::UsersRecipesLike;
use rocket::serde::json::Json;

#[get("/<recipe_id>/by/<user_id>")]
pub async fn read_recipe_by_login_user(
    conn: MyDatabase,
    recipe_id: usize,
    user_id: Option<usize>,
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

    match user_id {
        Some(user_id) => {
            conn.run(move |c| User::browse_recipe(c, user_id as i32, recipe_id as i32))
                .await;
        }
        _ => (),
    }

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

#[post("/delete/<recipe_id>/by/<user_id>")]
pub async fn delete(conn: MyDatabase, recipe_id: usize, user_id: usize) -> Json<bool> {
    let user = conn.run(move |c| User::from(c, user_id as i32)).await;
    let is_own_recipe = conn
        .run(move |c| Recipe::from(c, recipe_id as i32))
        .await
        .user_id
        == user_id as i32;
    let can_delete_own_recipe = user
        .user_type_with_authorities
        .authorities
        .iter()
        .any(move |authority| authority.hash == "delete_own_recipe".to_string());

    let can_delete_all_recipes = user
        .user_type_with_authorities
        .authorities
        .iter()
        .any(move |authority| authority.hash == "delete_all_recipes".to_string());

    let res = match (is_own_recipe && can_delete_own_recipe) || can_delete_all_recipes {
        true => conn.run(move |c| Recipe::delete(c, recipe_id as i32)).await,
        false => false,
    };
    Json(res)
}

/*
#[get("/likes_count/<recipe_id>")]
pub async fn likes_count(conn: MyDatabase, recipe_id: usize) -> Json<i32> {
    let recipe = conn.run(move |c| Recipe::from(c, recipe_id as i32)).await;
    let middle = UsersRecipesLike::belonging_to(&recipe)
        .load::<UsersRecipesLike>(conn)
        .unwrap();
    Json(1)
}
 */
#[get("/<recipe_id>")]
pub async fn read_recipe(conn: MyDatabase, recipe_id: usize) -> Json<RecipeWithItems> {
    let recipe = conn.run(move |c| Recipe::from(c, recipe_id as i32)).await;
    let ingredients = conn
        .run(move |c| Ingredient::from(c, recipe_id as i32))
        .await;
    let procedures = conn
        .run(move |c| Procedure::from(c, recipe_id as i32))
        .await;
    let tags = conn.run(move |c| Tag::from(c, recipe_id as i32)).await;
    let categories = conn.run(move |c| Category::from(c, recipe_id as i32)).await;

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

#[post("/update", format = "json", data = "<recipe>")]
pub async fn update(conn: MyDatabase, recipe: Json<UpdateRecipe>) -> Json<Recipe> {
    let update_recipe = recipe.into_inner();
    let ur = update_recipe.clone();
    let res = conn.run(move |c| Recipe::update(c, ur)).await;

    let rid = res.clone().id;
    let procedures = update_recipe
        .procedures
        .iter()
        .map(|procedure| NewProcedure {
            recipe_id: rid,
            number: procedure.number,
            discription: procedure.discription.clone(),
            image_path: procedure.image_path.clone(),
        })
        .collect::<Vec<NewProcedure>>();

    let ingredients = update_recipe
        .ingredients
        .iter()
        .map(|ingredient| NewIngredient {
            recipe_id: rid,
            name: ingredient.name.clone(),
            amount: ingredient.amount.clone(),
        })
        .collect::<Vec<NewIngredient>>();

    conn.run(move |c| Ingredient::create(c, &ingredients)).await;
    conn.run(move |c| Procedure::create(c, &procedures)).await;
    Json(res)
}

#[post("/", format = "json", data = "<recipe>")]
pub async fn create(conn: MyDatabase, recipe: Json<NewRecipeWithItems>) -> Json<Recipe> {
    let new_recipe_with_items = recipe.into_inner();
    let recipe = NewRecipe {
        user_id: new_recipe_with_items.user_id,
        title: new_recipe_with_items.title,
        thumbnail_path: new_recipe_with_items.thumbnail_path,
        discription: new_recipe_with_items.discription,
    };

    let tags = new_recipe_with_items.tags;
    let categories = new_recipe_with_items.categories;

    let recipe = conn.run(move |c| Recipe::create(recipe, c)).await;
    let rid = recipe.clone().id;
    let ingredients = new_recipe_with_items
        .ingredients
        .iter()
        .map(|ingredient| NewIngredient {
            recipe_id: rid,
            name: ingredient.name.clone(),
            amount: ingredient.amount.clone(),
        })
        .collect::<Vec<NewIngredient>>();

    let procedures = new_recipe_with_items
        .procedures
        .iter()
        .map(|procedure| NewProcedure {
            recipe_id: rid,
            number: procedure.number,
            discription: procedure.discription.clone(),
            image_path: procedure.image_path.clone(),
        })
        .collect::<Vec<NewProcedure>>();

    conn.run(move |c| Recipe::add_tags(c, rid, &tags)).await;
    conn.run(move |c| Recipe::add_categories(c, rid, &categories))
        .await;

    conn.run(move |c| Ingredient::create(c, &ingredients)).await;
    conn.run(move |c| Procedure::create(c, &procedures)).await;
    Json(recipe)
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
