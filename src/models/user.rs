use crate::schema::users;
use chrono;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[serde(crate = "rocket::serde")]
#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub user_type: i32,
    pub email: String,
    pub icon_path: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
    pub password: String,
    pub api_key: Option<String>,
}

#[serde(crate = "rocket::serde")]
#[derive(Deserialize, Insertable, FromForm, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub user_type: i32,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create(user: NewUser, conn: &PgConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .expect("Error creating new user");
        users::table.order(users::id.desc()).first(conn).unwrap()
    }

    pub fn read(conn: &PgConnection) -> Vec<User> {
        users::table.order(users::id).load::<User>(conn).unwrap()
    }

    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(conn).is_ok()
    }

    pub fn from(conn: &PgConnection, id: i32) -> User {
        users::table
            .filter(users::id.eq(id))
            .first::<User>(conn)
            .unwrap()
    }

    //    pub fn update()
}
