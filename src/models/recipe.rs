use crate::models::ingredient::Ingredient;
use crate::models::procedure::Procedure;
use crate::schema::recipes;
use chrono;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[serde(crate = "rocket::serde")]
#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Recipe {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub thumbnail_path: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
    pub discription: Option<String>,
}
#[serde(crate = "rocket::serde")]
#[derive(Serialize, Deserialize, Debug, Queryable)]
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
}

#[serde(crate = "rocket::serde")]
#[derive(Deserialize, Insertable, FromForm, Debug)]
#[table_name = "recipes"]
pub struct NewRecipe {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub thumbnail_path: Option<String>,
    pub discription: String,
}

impl Recipe {
    pub fn read(conn: &PgConnection) -> Vec<Recipe> {
        recipes::table
            .order(recipes::id)
            .load::<Recipe>(conn)
            .unwrap()
    }

    pub fn from(conn: &PgConnection, id: i32) -> Recipe {
        recipes::table
            .filter(recipes::id.eq(id))
            .first::<Recipe>(conn)
            .unwrap()
    }
}
