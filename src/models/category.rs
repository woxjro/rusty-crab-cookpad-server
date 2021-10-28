use crate::models::recipe::Recipe;
use crate::schema::categories;
use crate::schema::recipes_categories_categorization;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[serde(crate = "rocket::serde")]
#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub hash: String,
}

#[serde(crate = "rocket::serde")]
#[derive(Deserialize, Insertable, FromForm, Debug)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
    pub hash: String,
}

impl Category {
    /*
    pub fn from(conn: &PgConnection, recipe_id: i32) -> Vec<Category> {
        let recipe = Recipe::from(conn, recipe_id);

        RecipeCategoryCategorization::belonging_to(&recipe)
            .inner_join(categories::table)
            .select(categories::all_columns)
            .load::<Category>(conn)
            .unwrap()
    }
     */
}
#[serde(crate = "rocket::serde")]
#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[belongs_to(Category, foreign_key = "category_id")]
#[belongs_to(Recipe, foreign_key = "recipe_id")]
#[table_name = "recipes_categories_categorization"]
pub struct RecipeCategoryCategorization {
    pub id: i32,
    pub recipe_id: i32,
    pub category_id: i32,
}
