use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User<'r> {
    pub id: u8,
    pub name: &'r str,
    pub active: bool,
}
