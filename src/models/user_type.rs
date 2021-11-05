use crate::models::authority::Authority;
use crate::schema::user_types;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[derive(Identifiable, PartialEq, Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
#[table_name = "user_types"]
pub struct UserType {
    pub id: i32,
    pub name: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct UserTypeWithAuthorities {
    pub id: i32,
    pub name: String,
    pub hash: String,
    pub authorities: Vec<Authority>,
}

impl UserType {
    pub fn from(conn: &PgConnection, id: i32) -> UserType {
        let user_type = user_types::table
            .filter(user_types::id.eq(id))
            .first::<UserType>(conn)
            .unwrap();
        user_type
    }

    pub fn from_with_authorities(conn: &PgConnection, id: i32) -> UserTypeWithAuthorities {
        let user_type = user_types::table
            .filter(user_types::id.eq(id))
            .first::<UserType>(conn)
            .unwrap();
        let authorities = Authority::from(conn, id);
        UserTypeWithAuthorities {
            id: user_type.id,
            name: user_type.name,
            hash: user_type.hash,
            authorities: authorities,
        }
    }
}
