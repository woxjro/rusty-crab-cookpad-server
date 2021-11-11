use crate::models::recipe::Recipe;
use crate::schema::recipes_tags_tagging;
use crate::schema::tags;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[derive(Associations, Identifiable, Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub hash: String,
}

#[derive(Deserialize, Insertable, FromForm, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
    pub hash: String,
}

impl Tag {
    pub fn from(conn: &PgConnection, recipe_id: i32) -> Vec<Tag> {
        let recipe = Recipe::from(conn, recipe_id);
        let middle = RecipesTagsTagging::belonging_to(&recipe)
            .load::<RecipesTagsTagging>(conn)
            .unwrap();
        let tags = middle
            .iter()
            .filter(|e| {
                tags::table
                    .filter(tags::id.eq(e.tag_id))
                    .first::<Tag>(conn)
                    .is_ok()
            })
            .map(|e| {
                tags::table
                    .filter(tags::id.eq(e.tag_id))
                    .first::<Tag>(conn)
                    .unwrap()
            })
            .collect::<Vec<Tag>>();
        tags
    }
}

#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[belongs_to(Tag, foreign_key = "tag_id")]
#[belongs_to(Recipe, foreign_key = "recipe_id")]
#[table_name = "recipes_tags_tagging"]
pub struct RecipesTagsTagging {
    pub id: i32,
    pub recipe_id: i32,
    pub tag_id: i32,
}

#[derive(Deserialize, Insertable, FromForm, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "recipes_tags_tagging"]
pub struct NewRecipeTagTagging {
    pub recipe_id: i32,
    pub tag_id: i32,
}
