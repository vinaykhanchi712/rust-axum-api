use std::sync::Arc;
use axum::extract::State;
use axum::response::{IntoResponse, Json};
use axum::http::StatusCode;
use mongodb::bson::doc;
use mongodb::Database;
use serde_json::{json,Value};
use crate::models::{self,User,Schema};
use crate::utils::{encryption,jwt};
pub async fn authenticate(claims : jwt::Claims
                                                ) -> impl IntoResponse{
    match serde_json::to_value(claims){
        Ok(v) => (StatusCode::OK, Json(v)) , 
        Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "bad claims"})),
            ),
    }
}

pub async fn login(State(db): State<Arc<Database>>,
                   Json(payload): Json<models::User>
                                                            ) -> impl IntoResponse{

    let filter = doc! {"username": payload.username.replace('\"', "")};
    let collection = db.collection::<User>(&User::name());
    if let Ok(Some(user)) = collection.find_one(filter, None).await {
        let pwd = &payload.password.replace('\"', "");
        if encryption::validate(&user.password, pwd) {
            let jwt = match jwt::encode_user(user) {
                Ok(jwt) => jwt,
                Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({"error": "bad jwt"}))),
            };
            (
                StatusCode::OK,
                Json(json!({ "status": "logged in", "token": jwt})),
            )
        } else {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"status": "incorrect password"})),
            )
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "incorrect password"})),
        )
    }
}

/// Stores a new account in the database if body has been filled in correctly.
/// The password given will be stored encrypted.
pub async fn register(  State(db): State<Arc<Database>>,
                        Json(payload): Json<Value>
                                                            ) -> impl IntoResponse{

    let collection = db.collection::<User>(&User::name());
    let user = models::User::new(payload);

    match user {
        Ok(u) => {
            let mut new_user = u;
            new_user.password = encryption::hash(&new_user.password);
            if collection.insert_one(new_user, None).await.is_ok() {
                (
                    StatusCode::CREATED,
                    Json(json!({ "result": "Account created, login to receive token"})),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Could not store new user"})),
                )
            }
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        ),
    }
    
}