use axum::async_trait;
use mongodb::bson::DateTime;
use serde;
use serde_json::{Deserializer,Serializer,Value,Result};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;


#[async_trait]
pub trait Schema {
    /// Makes it easy to get the name of a struct when a Generic is used.
    fn name() -> String;
    /// The payload constructor itself, makes it possible to immediately pass JSON values to a struct object.
    fn new(payload: Value) -> Result<Self>
        where
            Self: Sized;
    /// Populate the data,
    /// for instance when a Model contains the ID of a different schema we want to populate the data
    async fn populate(&self, db: &Database) -> Value;
}


/// User object can easily be changed to JSON or mongoDB Documents.
/// This is because of the traits from serde Deserialize and Serialize
/// the id has to be an Option<ObjectId> type, simply because if we make a new struct we don't know its ObjectId yet/
/// This value is only filled in when a search has been done from mongoDB.
/// The user struct is also used for authentication and webtokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}

#[async_trait]
impl Schema for User {
    fn name() -> String {
        "users".to_string()
    }
    fn new(payload: Value) -> Result<Self> {
        serde_json::from_str(&payload.to_string())
    }

    async fn populate(&self, _: &Database) -> Value {
        serde_json::to_value(self).unwrap_or_else(|_| Value::Null)
    }
}

/// User object can easily be changed to JSON or mongoDB Documents.
/// This is because of the traits from serde Deserialize and Serialize
/// the id has to be an Option<ObjectId> type, simply because if we make a new struct we don't know its ObjectId yet/
/// This value is only filled in when a search has been done from mongoDB.
/// The author_id is used to check the ownership.
#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub content: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub release_date: DateTime,
    pub title: String,
    pub tags: Vec<String>,
    pub author_id: ObjectId,
}

#[async_trait]
impl Schema for Post {
    fn name() -> String {
        "posts".to_string()
    }
    fn new(payload: Value) -> Result<Self> {
        serde_json::from_str(&payload.to_string())
    }

    async fn populate(&self, _: &Database) -> Value {
        serde_json::to_value(self).unwrap_or_else(|_| Value::Null)
    }
}
/// User object can easily be changed to JSON or mongoDB Documents.
/// This is because of the traits from serde Deserialize and Serialize
/// the id has to be an Option<ObjectId> type, simply because if we make a new struct we don't know its ObjectId yet/
/// This value is only filled in when a search has been done from mongoDB.
/// The author_id is used to check the ownership.
#[derive(Debug, Deserialize, Serialize)]
pub struct Review {
    pub post: ObjectId,
    pub title: String,
    pub review: String,
    pub movie_title: String,
    pub author_id: ObjectId,
}

#[async_trait]
impl Schema for Review {
    fn name() -> String {
        "reviews".to_string()
    }
    fn new(payload: Value) -> Result<Self> {
        serde_json::from_str(&payload.to_string())
    }

    async fn populate(&self, db: &Database) -> Value {
        match serde_json::to_value(self) {
            Ok(mut value) => {
                let post_coll = db.collection::<Post>(&Post::name());
                match post_coll.find_one(doc! {"_id": self.post}, None).await {
                    Ok(Some(post)) => {
                        value["post"] = post.populate(&db).await;
                        value
                    }
                    _ => Value::Null,
                }
            }
            Err(_) => Value::Null,
        }
    }
}