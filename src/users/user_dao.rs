use super::user::User;

const USERS: [User; 2] = [
    User {
        id: 1,
        name: "Frikkie",
        active: true,
    },
    User {
        id: 2,
        name: "Pietie",
        active: true,
    },
];

pub struct UserDao {}
impl UserDao {
    pub fn list(&self) -> Vec<User<'static>> {
        USERS.to_vec()
    }

    pub fn get(&self, id: u8) -> Option<User<'static>> {
        USERS.into_iter().find(|u| u.id == id)
    }
}
