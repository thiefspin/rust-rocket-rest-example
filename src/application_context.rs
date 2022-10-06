use crate::users::{user_dao::UserDao, user_service::UserService};

pub struct ApplicationContext {
    pub user_service: UserService,
}

impl ApplicationContext {
    pub const fn new() -> ApplicationContext {
        let user_dao: UserDao = UserDao {};
        let user_service: UserService = UserService { dao: user_dao };
        ApplicationContext {
            user_service: user_service,
        }
    }
}
