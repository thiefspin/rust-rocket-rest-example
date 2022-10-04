#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};
use serde::Deserialize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct User<'r> {
    id: u8,
    name: &'r str,
    active: bool,
}

const USERS: [User; 2] = [User {
    id: 1,
    name: "Frikkie",
    active: true,
}, User {
    id: 2,
    name: "Pietie",
    active: true,
}];

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("{}{}", "Hello ", name)
}

#[get("/users")]
fn list() -> Json<[User<'static>; 2]> {
    let users = USERS;
    Json(users)
}

#[get("/users/<id>")]
fn get(id: u8) -> Option<Json<User<'static>>> {
    let user: Option<User> = USERS.into_iter().find(|u| u.id == id );
    user.map(|u| Json(u))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, list, get])
}
