use crate::schema::procedures;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Procedure {
    pub id: i32,
    pub recipe_id: i32,
    pub number: i32,
    pub discription: String,
    pub image_path: Option<String>,
}

#[derive(Deserialize, Insertable, FromForm, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "procedures"]
pub struct NewProcedure {
    pub recipe_id: i32,
    pub number: i32,
    pub discription: String,
    pub image_path: Option<String>,
}

impl Procedure {
    pub fn from(conn: &PgConnection, recipe_id: i32) -> Vec<Procedure> {
        procedures::table
            .filter(procedures::recipe_id.eq(recipe_id))
            .order_by(procedures::number)
            .load::<Procedure>(conn)
            .unwrap()
    }
}
