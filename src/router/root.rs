use std::sync::Arc;

use axum::{Router,middleware};
use mongodb::Database;
use super::{user_router,post_router,review_router};
use crate::middleware::{logging,ownership};

pub fn root_router(db : Database) -> Router{
    let state = Arc::new(db);
    Router::new()
        .nest("/users", user_router::routes())
        .nest("/posts", post_router::routes())
        .nest("/reviews", review_router::routes())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            ownership::check_owner,
        ))
        .layer(middleware::from_fn(logging::logger))
        .with_state(state)
}