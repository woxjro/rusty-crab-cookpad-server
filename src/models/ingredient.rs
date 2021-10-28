use crate::schema::ingredients;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[serde(crate = "rocket::serde")]
#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub recipe_id: i32,
    pub name: String,
    pub amount: String,
}

#[serde(crate = "rocket::serde")]
#[derive(Deserialize, Insertable, FromForm, Debug)]
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
