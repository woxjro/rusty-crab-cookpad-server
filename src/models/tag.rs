use crate::models::recipe::Recipe;
use crate::schema::recipes_tags_tagging;
use crate::schema::tags;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[serde(crate = "rocket::serde")]
#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub hash: String,
}

#[serde(crate = "rocket::serde")]
#[derive(Deserialize, Insertable, FromForm, Debug)]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
    pub hash: String,
}

impl Tag {
    pub fn from(conn: &PgConnection, recipe_id: i32) -> Vec<Tag> {
        tags::table
            //            .filter(tags::recipe_id.eq(recipe_id))
            .load::<Tag>(conn)
            .unwrap()
    }
}
#[serde(crate = "rocket::serde")]
#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[belongs_to(Tag, foreign_key = "tag_id")]
#[belongs_to(Recipe, foreign_key = "recipe_id")]
#[table_name = "recipes_tags_tagging"]
pub struct RecipesTagsTagging {
    pub id: i32,
    pub recipe_id: i32,
    pub tag_id: i32,
}
