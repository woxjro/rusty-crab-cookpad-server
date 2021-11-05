use crate::models::recipe::Recipe;
use crate::models::user_type::{UserType, UserTypeWithAuthorities};
use crate::schema::recipes;
use crate::schema::users;
use crate::schema::users_recipes_browsing_history;
use crate::schema::users_recipes_like;
use chrono;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[derive(Identifiable, Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
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

#[derive(Deserialize, Insertable, FromForm, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub user_type: i32,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct UserWithAuthorities {
    pub id: i32,
    pub name: String,
    pub user_type_with_authorities: UserTypeWithAuthorities,
    pub email: String,
    pub icon_path: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
    pub password: String,
    pub api_key: Option<String>,
}

impl User {
    fn user_type_with_authorities(&self, conn: &PgConnection) -> UserTypeWithAuthorities {
        let user_type_id = self.user_type;
        UserType::from_with_authorities(conn, user_type_id)
    }

    pub fn create(user: NewUser, conn: &PgConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .expect("Error creating new user");
        users::table.order(users::id.desc()).first(conn).unwrap()
    }

    pub fn read(conn: &PgConnection) -> Vec<UserWithAuthorities> {
        let users = users::table.order(users::id).load::<User>(conn).unwrap();
        let mut res = vec![];
        for user in users {
            let user_type_with_authorities = user.user_type_with_authorities(conn);
            let r = UserWithAuthorities {
                id: user.id,
                name: user.name,
                user_type_with_authorities: user_type_with_authorities,
                email: user.email,
                icon_path: user.icon_path,
                created_at: user.created_at,
                updated_at: user.updated_at,
                password: user.password,
                api_key: user.api_key,
            };
            res.push(r);
        }
        res
    }

    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(conn).is_ok()
    }

    pub fn from(conn: &PgConnection, id: i32) -> UserWithAuthorities {
        let user = users::table
            .filter(users::id.eq(id))
            .first::<User>(conn)
            .unwrap();
        let user_type_with_authorities = user.user_type_with_authorities(conn);
        UserWithAuthorities {
            id: user.id,
            name: user.name,
            user_type_with_authorities: user_type_with_authorities,
            email: user.email,
            icon_path: user.icon_path,
            created_at: user.created_at,
            updated_at: user.updated_at,
            password: user.password,
            api_key: user.api_key,
        }
    }

    pub fn liked_recipes(conn: &PgConnection, id: i32) -> Vec<Recipe> {
        let user = users::table
            .filter(users::id.eq(id))
            .first::<User>(conn)
            .unwrap();
        let middle = UsersRecipesLike::belonging_to(&user)
            .load::<UsersRecipesLike>(conn)
            .unwrap();
        let recipes = middle
            .iter()
            .map(|e| {
                recipes::table
                    .filter(recipes::id.eq(e.recipe_id))
                    .first::<Recipe>(conn)
                    .unwrap()
            })
            .collect::<Vec<Recipe>>();
        recipes
    }

    pub fn recipes_browsing_history(conn: &PgConnection, id: i32) -> Vec<Recipe> {
        let user = users::table
            .filter(users::id.eq(id))
            .first::<User>(conn)
            .unwrap();
        let middle = UsersRecipesBrowsingHistory::belonging_to(&user)
            .order(users_recipes_browsing_history::created_at.desc())
            .load::<UsersRecipesBrowsingHistory>(conn)
            .unwrap();
        let recipes = middle
            .iter()
            .map(|e| {
                recipes::table
                    .filter(recipes::id.eq(e.recipe_id))
                    .first::<Recipe>(conn)
                    .unwrap()
            })
            .collect::<Vec<Recipe>>();
        recipes
    }
}

#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Recipe, foreign_key = "recipe_id")]
#[table_name = "users_recipes_like"]
pub struct UsersRecipesLike {
    pub id: i32,
    pub user_id: i32,
    pub recipe_id: i32,
}

#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Recipe, foreign_key = "recipe_id")]
#[table_name = "users_recipes_browsing_history"]
pub struct UsersRecipesBrowsingHistory {
    pub id: i32,
    pub user_id: i32,
    pub recipe_id: i32,
    pub created_at: chrono::NaiveDateTime,
}
