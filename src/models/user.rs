use warp::{Rejection, Reply};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::utils::establish_connection;
use crate::schema::users as users_schema;

#[derive(Queryable, Serialize)]
pub struct User {
    id: u64,
    name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users_schema"]
pub struct UserData {
    name: String,
}

impl User {
    pub fn read() -> Vec<User> {
        let connection = establish_connection();    
        let users = users_schema::dsl::users
          .load::<User>(&connection)
          .expect("Error loading users");
          
        return users;
    }

    pub fn create(body: UserData) -> Result<impl Reply, Rejection> {
        let connection = establish_connection();    
        diesel::insert_into(users_schema::dsl::users)
        .values(body)
        .execute(&connection)
        .expect("Error saving new user");

        return Ok(warp::http::StatusCode::OK);
    }
    
    pub fn update(id: u64, body: UserData) -> Result<impl Reply, Rejection> {
        let connection = establish_connection();    
        diesel::update(users_schema::dsl::users.find(id))
        .set(users_schema::name.eq(body.name))
        .execute(&connection)
        .expect("Error saving new user");

        return Ok(warp::http::StatusCode::OK);
    }

    pub fn delete(id: u64) -> Result<impl Reply, Rejection> {
        let connection = establish_connection();    
        diesel::delete(users_schema::dsl::users.find(id))
        .execute(&connection)
        .expect("Error saving new user");

        return Ok(warp::http::StatusCode::OK);
    }
}
