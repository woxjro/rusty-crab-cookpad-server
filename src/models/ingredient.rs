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

#[derive(Serialize, Deserialize, Insertable, FromForm, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "ingredients"]
pub struct NewIngredient {
    pub recipe_id: i32,
    pub name: String,
    pub amount: String,
}

#[derive(Clone, Serialize, Deserialize, Insertable, FromForm, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "ingredients"]
pub struct NewApiIngredient {
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

    pub fn create(conn: &PgConnection, ingredients: &Vec<NewIngredient>) -> () {
        for ingredient in ingredients {
            diesel::insert_into(ingredients::table)
                .values(ingredient)
                .execute(conn)
                .expect("Error creating new ingredient");
        }
    }
}
