use axum::{
    routing::{get, post},
    Router,
};

use crate::repository::Repository;

mod auth;
mod country;

pub fn make_router(app_state: Repository) -> Router {
    let city_router = Router::new()
        .route("/cities/:city_name", get(country::get_city_handler))
        .route("/cities", post(country::post_city_handler));

    let auth_router = Router::new()
        .route("/signup", post(auth::sign_up))
        .route("/login", post(auth::login));

    let ping_router = Router::new().route("/ping", get(|| async { "pong" })); // [!code ++]

    Router::new()
        .nest("/", city_router)
        .nest("/", auth_router)
        .nest("/", ping_router) // [!code ++]
        .with_state(app_state)
}