#![allow(dead_code)]

#[macro_use]
extern crate rocket;

mod application_context;
mod controllers;
mod users;
mod utils;

use application_context::ApplicationContext;
use rocket::serde::json::Json;
use rocket::Request;
use utils::errors::{InternalServerError, NotFoundError};

use crate::controllers::user_controller::{get_user, list_users};

const CTX: ApplicationContext = ApplicationContext::new();

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("{}{}", "Hello ", name)
}

#[catch(404)]
fn not_found(req: &Request) -> Json<NotFoundError> {
    let not_found = NotFoundError {
        status: 404,
        description: format!("No path matching '{}'", req.uri()),
    };
    Json(not_found)
}

#[catch(500)]
fn internal_error() -> Json<InternalServerError> {
    Json(InternalServerError {
        status: 500,
        description: format!("Well thats embarrassing.."),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .mount("/api/users", routes![list_users, get_user])
        .register("/", catchers![not_found])
}
