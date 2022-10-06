use super::{user::User, user_dao::UserDao};

pub struct UserService {
    pub dao: UserDao,
}

impl UserService {
    pub fn list(&self) -> Vec<User<'static>> {
        self.dao.list()
    }

    pub fn get(&self, id: u8) -> Option<User<'static>> {
        self.dao.get(id)
    }
}
