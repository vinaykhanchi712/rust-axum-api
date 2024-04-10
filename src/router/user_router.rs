use mongodb::Database;
use std::sync::Arc;
use axum::{Router , routing::get , routing::post};
use crate::controller::{auth_controller,generic_controller};

pub fn routes() -> Router<Arc<Database>>{
    Router::new()
        .route("/", get(generic_controller::get_all::<models::User>))
        .route("/:id", get(generic_controller::get_by_id::<models::User>))
        .route("/login", post(auth_controller::login))
        .route("/auth", get(auth_controller::authenticate))
        .route("/register", post(auth_controller::register))
}