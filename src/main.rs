use warp_sample::routes::users_api;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    // let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // warp::serve(hello).run(([0, 0, 0, 0], 3000)).await;
    warp::serve(users_api())
        .run(([127, 0, 0, 1], 3000))
        .await;
}