use warp::{Filter, filters::BoxedFilter, Reply};
use crate::controllers::user_controller;
use crate::models::user::UserData;

pub fn users_api() -> BoxedFilter<(impl Reply,)> {
    get_users()
    .or(post_user())
    .or(put_user())
    .or(delete_user())
    .boxed()
}

/// PathからUserIdを取り出す部品
fn user_id() -> BoxedFilter<(u64,)> {
    warp::path::param().boxed()
}

fn get_users() -> BoxedFilter<(impl Reply,)> {
    warp::path("users")
        .and(warp::get())    // HTTP GETメソッドを指定
        .and_then(move || user_controller::get_users_handler())    // Handlerを呼び出す
        .boxed()
}

/// post_user_handlerを呼び出すための部品
fn post_user() -> BoxedFilter<(impl Reply,)> {
    warp::path("users")
        .and(warp::post())    // HTTP POSTメソッドを指定
        .and(warp::body::json::<UserData>())    // Request Bodyに含まれたJSONを取り出しUser型へ変換
        .and_then(move |body| user_controller::post_user_handler(body))    // Handlerを呼び出す
        .boxed()
}

fn put_user() -> BoxedFilter<(impl Reply,)> {
    warp::path("users")
        .and(user_id())
        .and(warp::put())    // HTTP POSTメソッドを指定
        .and(warp::body::json::<UserData>())    // Request Bodyに含まれたJSONを取り出しUser型へ変換
        .and_then(move |id, body| user_controller::put_user_handler(id, body))    // Handlerを呼び出す
        .boxed()
}

fn delete_user() -> BoxedFilter<(impl Reply,)> {
    warp::path("users")
        .and(user_id())
        .and(warp::delete())    // HTTP POSTメソッドを指定
        .and_then(move |id| user_controller::delete_user_handler(id))    // Handlerを呼び出す
        .boxed()
}