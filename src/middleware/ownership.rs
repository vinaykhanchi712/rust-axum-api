use std::sync::Arc;
use std::str::FromStr;
use axum::extract::State;
use axum::http;
use axum::http::{Method, Request , StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use mongodb::{bson::{doc, oid::ObjectId}, Collection, Database};
use serde_json::Value;
use crate::utils;

pub async fn check_owner<B>(State(state) : State<Arc<Database>>,
                            request : Request<B>,
                            next : Next<B>,                    ) -> impl IntoResponse {

    let db = state;
    let auth_header = request
                            .headers()
                            .get(http::header::AUTHORIZATION)
                            .and_then(|x| x.to_str().ok() );

    if request.method() == Method::PUT || request.method() == Method::DELETE {
        return match auth_header {
            Some(auth_header)
            if is_object_owner(auth_header, &request.uri().to_string(), &db).await =>
                {
                    next.run(request).await
                }
            _ => (StatusCode::UNAUTHORIZED).into_response(),
        }
    }
    next.run(request).await

}

async fn is_object_owner(auth_header : &str , uri : &str , db : &Database ) -> bool {
    let auth_header = *auth_header.split(' ').collect::<Vec<&str>>().get(1).unwrap_or(&"");
    let claims = utils::jwt::decode_jwt(auth_header);

    let user_id = if let Ok(c) = claims{
        if let Some(id)= c.user.id{
            id.to_string()
        }else{
            return false;
        }
    }else {
        return false;
    };

    let search_info = get_model_and_id(uri);
    let (model_name, object_id) = if let (Some(model_name), Some(object_id)) = search_info {
        (model_name, object_id)
    } else {
        return false;
    };
    let collection = db.collection::<Value>(&model_name);
    db_lookup(&collection, &object_id, &user_id).await


}

fn get_model_and_id(uri : &str) -> (Option<String>, Option<String>){
    let parts = uri.split('/').collect::<Vec<&str>>();
    let model_name = parts.get(1).map(|s| s.to_owned().to_owned());
    let object_id = parts.get(2).map(|s| s.to_owned().to_owned());
    (model_name, object_id)
}

async fn db_lookup(collection: &Collection<Value>, object_id: &str, user_id: &str) -> bool{
    let object_id = if let Ok(obj_id) = mongodb::bson::oid::ObjectId::from_str(object_id) {
        obj_id
    } else {
        return false;
    };
    let filter = doc! {"_id": object_id};
    let object = collection.find_one(filter, None).await;
    if let Ok(Some(object)) = object {
        match serde_json::from_value::<ObjectId>(object["author_id"].clone()) {
            Ok(id) => id.to_string() == user_id,
            Err(_) => false,
        }
    } else {
        false
    }
}