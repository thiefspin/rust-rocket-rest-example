#[path = "../users/mod.rs"]
mod users;

use rocket::serde::json::Json;

use crate::{users::user::User, CTX};

#[get("/users")]
pub fn list() -> Json<Vec<User<'static>>> {
    let users = CTX.user_service.list();
    Json(users)
}

#[get("/users/<id>")]
pub fn get(id: u8) -> Option<Json<User<'static>>> {
    let user: Option<User> = CTX.user_service.get(id);
    user.map(|u| Json(u))
}
