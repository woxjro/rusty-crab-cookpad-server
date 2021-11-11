use crate::models::category::Category;
use crate::models::category::NewRecipeCategoryCategorization;
use crate::models::ingredient::{Ingredient, NewApiIngredient};
use crate::models::procedure::{NewApiProcedure, Procedure};
use crate::models::tag::{NewRecipeTagTagging, Tag};
use crate::models::user::UsersRecipesLike;
use crate::schema::ingredients;
use crate::schema::procedures;
use crate::schema::recipes;
use crate::schema::recipes_categories_categorization;
use crate::schema::recipes_tags_tagging;
use crate::schema::users_recipes_browsing_history;
use crate::schema::users_recipes_like;
use chrono;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Identifiable, PartialEq, Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub thumbnail_path: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
    pub discription: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct RecipeWithItems {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub thumbnail_path: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
    pub discription: Option<String>,
    pub ingredients: Vec<Ingredient>,
    pub procedures: Vec<Procedure>,
    pub tags: Vec<Tag>,
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Insertable, FromForm, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "recipes"]
pub struct NewRecipe {
    pub user_id: i32,
    pub title: String,
    pub thumbnail_path: Option<String>,
    pub discription: String,
}

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct NewRecipeWithItems {
    pub user_id: i32,
    pub title: String,
    pub thumbnail_path: Option<String>,
    pub discription: String,
    pub ingredients: Vec<NewApiIngredient>,
    pub procedures: Vec<NewApiProcedure>,
    pub tags: Vec<i32>,
    pub categories: Vec<i32>,
}

impl Recipe {
    pub fn create(recipe: NewRecipe, conn: &PgConnection) -> Recipe {
        diesel::insert_into(recipes::table)
            .values(&recipe)
            .execute(conn)
            .expect("Error creating new user");
        recipes::table
            .order(recipes::id.desc())
            .first(conn)
            .unwrap()
    }

    pub fn read(conn: &PgConnection) -> Vec<RecipeWithItems> {
        let recipes = recipes::table
            .order(recipes::id)
            .load::<Recipe>(conn)
            .unwrap();

        let mut res = vec![];
        for recipe in recipes {
            let id = recipe.id;
            let ingredients = Ingredient::from(conn, id);
            let procedures = Procedure::from(conn, id);
            let tags = Tag::from(conn, id);
            let categories = Category::from(conn, id);
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
                tags: tags,
                categories: categories,
            };
            res.push(r);
        }
        res
    }

    pub fn delete(conn: &PgConnection, recipe_id: i32) -> bool {
        //ingredient
        let _ = diesel::delete(ingredients::table.filter(ingredients::recipe_id.eq(recipe_id)))
            .execute(conn)
            .is_ok();
        //procedures
        let _ = diesel::delete(procedures::table.filter(procedures::recipe_id.eq(recipe_id)))
            .execute(conn)
            .is_ok();
        //category
        let _ = diesel::delete(
            recipes_categories_categorization::table
                .filter(recipes_categories_categorization::recipe_id.eq(recipe_id)),
        )
        .execute(conn)
        .is_ok();
        //tag
        let _ = diesel::delete(
            recipes_tags_tagging::table.filter(recipes_tags_tagging::recipe_id.eq(recipe_id)),
        )
        .execute(conn)
        .is_ok();
        //browsing history
        let _ = diesel::delete(
            users_recipes_browsing_history::table
                .filter(users_recipes_browsing_history::recipe_id.eq(recipe_id)),
        )
        .execute(conn)
        .is_ok();
        //like
        let _ = diesel::delete(
            users_recipes_like::table.filter(users_recipes_like::recipe_id.eq(recipe_id)),
        )
        .execute(conn)
        .is_ok();

        diesel::delete(recipes::table.find(recipe_id))
            .execute(conn)
            .is_ok()
    }

    pub fn read_with_query(
        conn: &PgConnection,
        user_id: Option<i32>,
        tag_id: Option<i32>,
        category_id: Option<i32>,
    ) -> Vec<RecipeWithItems> {
        let mut query = recipes::table.into_boxed();
        if let Some(id) = user_id {
            query = query.filter(recipes::user_id.eq(id));
        }
        let recipes = query.order(recipes::id).load::<Recipe>(conn).unwrap();

        let mut res = vec![];
        for recipe in recipes {
            let id = recipe.id;
            let ingredients = Ingredient::from(conn, id);
            let procedures = Procedure::from(conn, id);
            let tags = Tag::from(conn, id);
            let categories = Category::from(conn, id);

            if let Some(id) = tag_id {
                let have = tags.iter().any(|tag| tag.id == id);
                if !have {
                    continue;
                }
            }

            if let Some(id) = category_id {
                let have = categories.iter().any(|category| category.id == id);
                if !have {
                    continue;
                }
            }

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
                tags: tags,
                categories: categories,
            };
            res.push(r);
        }
        res
    }

    pub fn count_likes(conn: &PgConnection, recipe_id: i32) -> usize {
        let likes = users_recipes_like::table
            .filter(users_recipes_like::recipe_id.eq(recipe_id))
            .load::<UsersRecipesLike>(conn)
            .unwrap_or(vec![]);
        likes.len()
    }

    pub fn from(conn: &PgConnection, id: i32) -> Recipe {
        let recipe = recipes::table
            .filter(recipes::id.eq(id))
            .first::<Recipe>(conn)
            .unwrap();
        recipe
    }

    pub fn search(conn: &PgConnection, words: Vec<String>) -> Vec<Recipe> {
        let mut query = recipes::table.into_boxed();
        for word in words {
            query = query
                .or_filter(recipes::title.like(format!("%{}%", word).to_string()))
                .or_filter(recipes::discription.like(format!("%{}%", word).to_string()));
        }
        query.load::<Recipe>(conn).unwrap()
    }

    pub fn add_tags(conn: &PgConnection, recipe_id: i32, tag_ids: &Vec<i32>) -> () {
        for &tag_id in tag_ids {
            let recipe_tag = NewRecipeTagTagging {
                recipe_id: recipe_id,
                tag_id: tag_id,
            };
            diesel::insert_into(recipes_tags_tagging::table)
                .values(&recipe_tag)
                .execute(conn)
                .unwrap();
        }
    }

    pub fn add_categories(conn: &PgConnection, recipe_id: i32, category_ids: &Vec<i32>) -> () {
        for &category_id in category_ids {
            let recipe_category = NewRecipeCategoryCategorization {
                recipe_id: recipe_id,
                category_id: category_id,
            };
            diesel::insert_into(recipes_categories_categorization::table)
                .values(&recipe_category)
                .execute(conn)
                .unwrap();
        }
    }
}
