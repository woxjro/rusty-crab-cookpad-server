use crate::models::user_type::UserType;
use crate::schema::authorities;
use crate::schema::user_types_authorities_ownership;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[derive(Associations, Identifiable, Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
#[table_name = "authorities"]
pub struct Authority {
    pub id: i32,
    pub name: String,
    pub hash: String,
}

impl Authority {
    pub fn from(conn: &PgConnection, user_type_id: i32) -> Vec<Authority> {
        let user_type = UserType::from(conn, user_type_id);
        let middle = UserTypesAuthoritiesOwnership::belonging_to(&user_type)
            .load::<UserTypesAuthoritiesOwnership>(conn)
            .unwrap();
        let authorities = middle
            .iter()
            .map(|e| {
                authorities::table
                    .filter(authorities::id.eq(e.authority_id))
                    .first::<Authority>(conn)
                    .unwrap()
            })
            .collect::<Vec<Authority>>();
        authorities
    }
}
#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[belongs_to(UserType, foreign_key = "user_type_id")]
#[belongs_to(Authority, foreign_key = "authority_id")]
#[table_name = "user_types_authorities_ownership"]
pub struct UserTypesAuthoritiesOwnership {
    pub id: i32,
    pub user_type_id: i32,
    pub authority_id: i32,
}
