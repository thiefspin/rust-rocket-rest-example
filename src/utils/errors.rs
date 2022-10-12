use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NotFoundError {
    pub status: u16,
    pub description: String
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InternalServerError {
    pub status: u16,
    pub description: String
}