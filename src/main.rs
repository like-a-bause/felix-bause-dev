use axum::{
    Router,
};

use tower_http::{
    services::{ServeDir}
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().nest_service("/", ServeDir::new("public"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}