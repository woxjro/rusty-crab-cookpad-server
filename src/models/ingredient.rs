use crate::schema::ingredients;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Ingredient {
    pub id: i32,
    pub recipe_id: i32,
    pub name: String,
    pub amount: String,
}

#[derive(Deserialize, Insertable, FromForm, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "ingredients"]
pub struct NewIngredient {
    pub recipe_id: i32,
    pub name: String,
    pub amount: String,
}

impl Ingredient {
    pub fn from(conn: &PgConnection, recipe_id: i32) -> Vec<Ingredient> {
        ingredients::table
            .filter(ingredients::recipe_id.eq(recipe_id))
            .load::<Ingredient>(conn)
            .unwrap()
    }
}
