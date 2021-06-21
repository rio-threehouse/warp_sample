use warp::{Rejection, Reply};
use crate::models::user;

pub async fn get_users_handler() -> Result<impl Reply, Rejection> {
  let users = user::User::read();
  Ok(warp::reply::json(&users))
}

pub async fn post_user_handler(body: user::UserData) -> Result<impl Reply, Rejection> {
  return user::User::create(body);
}

pub async fn put_user_handler(id:u64, body: user::UserData) -> Result<impl Reply, Rejection> {
  return user::User::update(id, body);
}

pub async fn delete_user_handler(id:u64) -> Result<impl Reply, Rejection> {
  return user::User::delete(id);
}