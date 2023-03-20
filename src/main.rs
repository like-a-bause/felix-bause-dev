use axum::{Router};
use axum::routing::get;

use tower_http::{
    services::{ServeDir},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use jwt_authorizer::{AuthError, JwtAuthorizer, JwtClaims};
use jwt_authorizer::layer::JwtSource;
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "felix_bause_dev=debug,tower_http=debug,jwt_authorizer=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // prod:
    let jwt_auth: JwtAuthorizer<User> =
        JwtAuthorizer::from_jwks_url("https://d253da95-fbf2-4cd7-a61a-8d4bbaf281d3.hanko.io/.well-known/jwks.json");

    // localhost:3000
    // let jwt_auth: JwtAuthorizer<User> =
    //     JwtAuthorizer::from_jwks_url("https://576702ec-9052-4732-af6a-19e40a8b57af.stg.hanko.io/.well-known/jwks.json");

    // subroute for privileged api calls
    let privileged_routes = Router::new()
        .route("/", get(protected))
        .layer(jwt_auth.layer(JwtSource::Cookie(String::from("hanko"))).await.unwrap())
        .layer(CookieManagerLayer::new());

    // subroute for /api
    let api_routes = Router::new()
        .nest("/privileged", privileged_routes)
        .route("/health", get(|| async{"I'm Ok, thank you."}));

    // build our application with a single route
    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http());

    tracing::info!("Starting server...");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// struct representing the authorized caller, deserializable from JWT claims
#[derive(Debug, Deserialize, Clone)]
struct User {
    sub: String,
}

// proteced handler with user injection (mapping some jwt claims)
async fn protected(JwtClaims(user): JwtClaims<User>) -> Result<String, AuthError> {
    Ok(format!("Welcome: {}", user.sub))
}