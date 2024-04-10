
//! # axum-test:
//! Axum test is a sample project for testing how Rust can be used to make a RESTFUL Api that responds with JSON,
//! uses webtokens and stores data by using a database.
//! The goals set for this project were:
//! - Connect to a database
//! - CRUD requests
//! - several models
//! - encrypt passwords of users
//! - use jsonwebtokens for requests
//! - middleware
//! - a model with complex datatypes (time, lists e.g.)
//! - only objects can be deleted/edited by the author
//!
//! Apart from these goals, this project also succeeded into create a working generic controller.

use std::env;
use std::net::SocketAddr;
use axum;

mod mongo;
mod router;
mod controller;
mod utils;
mod models;
mod middleware;


#[tokio::main]
async fn main() {
    
    let db = mongo::database().await;
    let router = router::root::root_router(db);
    let port = env::var("PORT").unwrap_or(String::from("8000"));
    let addr = ["0.0.0.0:", &port].concat();
    let server : SocketAddr = addr.parse().expect("Could not parse the socket address");
    
    let axum_server = axum::Server::bind(&server).serve(router.into_make_service()).await;
    if(axum_server).is_err() {
        panic!("could not start the server");
    }else{
        println!("Server started on addr :{} ", addr.to_string());
    }
}
