#![allow(dead_code)]

#[macro_use]
extern crate rocket;

mod application_context;
mod controllers;
mod users;

use crate::application_context::ApplicationContext;
use crate::controllers::user_controller::{get, list};

const CTX: ApplicationContext = ApplicationContext::new();

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("{}{}", "Hello ", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, list, get])
}
