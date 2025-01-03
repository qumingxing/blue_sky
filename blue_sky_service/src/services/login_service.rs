use blue_sky_dao::dao::jdbc_template::BaseDao;
use blue_sky_dao::get_user_dao;
use blue_sky_entity::dto::user::User;
use sha2::{Digest, Sha256, Sha512};
use crate::utils::get_hex;

pub struct LoginService {
    username: String,
    password: String,
}

impl LoginService {
    pub fn new(username: String, password: String) -> LoginService {
        LoginService { username, password }
    }

    pub fn login(&self) -> bool {
        let user = &User {
            user_name: self.username.clone(),
            ..Default::default()
        };
        let user_dao = get_user_dao();
        let res = user_dao.get(user);
        if let Ok(u) = res {
            if get_hex(self.password.as_str()) == u.password {
                return true;
            }
        }
        false
    }
}
