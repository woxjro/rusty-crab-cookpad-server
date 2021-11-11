use crate::models::recipe::Recipe;
use crate::schema::categories;
use crate::schema::recipes_categories_categorization;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub hash: String,
}

#[derive(Deserialize, Insertable, FromForm, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
    pub hash: String,
}

impl Category {
    pub fn from(conn: &PgConnection, recipe_id: i32) -> Vec<Category> {
        let recipe = Recipe::from(conn, recipe_id);
        let middle = RecipeCategoryCategorization::belonging_to(&recipe)
            .load::<RecipeCategoryCategorization>(conn)
            .unwrap();
        let categories = middle
            .iter()
            .filter(|e| {
                categories::table
                    .filter(categories::id.eq(e.category_id))
                    .first::<Category>(conn)
                    .is_ok()
            })
            .map(|e| {
                categories::table
                    .filter(categories::id.eq(e.category_id))
                    .first::<Category>(conn)
                    .unwrap()
            })
            .collect::<Vec<Category>>();
        categories
    }
}
#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[belongs_to(Category, foreign_key = "category_id")]
#[belongs_to(Recipe, foreign_key = "recipe_id")]
#[table_name = "recipes_categories_categorization"]
pub struct RecipeCategoryCategorization {
    pub id: i32,
    pub recipe_id: i32,
    pub category_id: i32,
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "recipes_categories_categorization"]
pub struct NewRecipeCategoryCategorization {
    pub recipe_id: i32,
    pub category_id: i32,
}
