use crate::models::category::Category;
use crate::models::ingredient::Ingredient;
use crate::models::procedure::Procedure;
use crate::models::tag::Tag;
use crate::schema::recipes;
use chrono;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};
#[derive(Identifiable, PartialEq, Serialize, Deserialize, Debug, Queryable)]
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
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub thumbnail_path: Option<String>,
    pub discription: String,
}

impl Recipe {
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

    pub fn read_with_query(
        conn: &PgConnection,
        user_id: Option<i32>,
        tag_id: Option<i32>,
        category_id: Option<i32>,
    ) -> Vec<RecipeWithItems> {
        let recipes = match user_id {
            Some(id) => {
                (recipes::table
                    .order(recipes::id)
                    .filter(recipes::user_id.eq(id))
                    .load::<Recipe>(conn)
                    .unwrap())
            }
            _ => {
                (recipes::table
                    .order(recipes::id)
                    .load::<Recipe>(conn)
                    .unwrap())
            }
        };
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
            query = query.filter(recipes::title.like(word));
        }
        query.load::<Recipe>(conn).unwrap()
    }
}
